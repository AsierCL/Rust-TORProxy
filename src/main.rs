use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, copy, split},
    net::{TcpListener, TcpStream},
    spawn, time::{sleep, Duration},
};
use tokio_socks::tcp::Socks5Stream;
use anyhow::Result;

// Local proxy listens here
const LOCAL_ADDR: &str = "127.0.0.1:12345";
// Tor's SOCKS port to launch (we choose a non-default to avoid conflicts)
const TOR_SOCKS_ADDR: &str = "127.0.0.1:9050";

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Launch Tor subprocess
    let tor_dir = std::env::temp_dir().join("rust_tor_proxy_data");
    std::fs::create_dir_all(&tor_dir)?;
    println!("Starting Tor with DataDirectory {:?}...", tor_dir);

    // Wait for Tor to accept SOCKS connections
    for i in 0..10 {
        if TcpStream::connect(TOR_SOCKS_ADDR).await.is_ok() {
            println!("Tor is ready at {}", TOR_SOCKS_ADDR);
            break;
        }
        println!("Waiting for Tor startup... ({}/10)", i+1);
        sleep(Duration::from_secs(1)).await;
        if i == 9 {
            return Err(anyhow::anyhow!("Tor did not start in time"));
        }
    }

    // 2. Start local SOCKS5 proxy
    let listener = TcpListener::bind(LOCAL_ADDR).await?;
    println!("Proxy listening on {}", LOCAL_ADDR);

    // 3. Accept and handle connections
    loop {
        let (client, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
        spawn(async move {
            if let Err(e) = handle_client(client).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
    // Optionally, to clean up Tor: tor_proc.kill().await?;
}

async fn handle_client(mut client: TcpStream) -> Result<()> {
    // SOCKS5 handshake
    let mut header = [0u8; 2]; // Read 2 first bytes
    client.read_exact(&mut header).await?;
    if header[0] != 0x05 {  // 0x05 = SOCKS5
        return Err(anyhow::anyhow!("Unsupported SOCKS version"));
    }
    let n_methods = header[1] as usize;
    let mut methods = vec![0u8; n_methods]; 
    client.read_exact(&mut methods).await?;
    // No auth
    client.write_all(&[0x05, 0x00]).await?; // Respond with no authentication required

    // SOCKS5 request
    let mut req = [0u8; 4];
    client.read_exact(&mut req).await?;
    if req[1] != 0x01 { // 0x01 = CONNECT, 0x02 = BIND, 0x03 = UDP ASSOCIATE
        return Err(anyhow::anyhow!("Only CONNECT supported"));
    }

    // Parse target address and port
    let target = match req[3] { // Address type: 0x01 = IPv4, 0x03 = Domain, 0x04 = IPv6
        0x01 => { // IPv4
            // Read 4 bytes for IPv4 address
            let mut ip = [0u8; 4];
            client.read_exact(&mut ip).await?;
            let addr = std::net::IpAddr::from(ip);
            // Read 2 bytes for port (big-endian)
            let mut port_b = [0u8; 2]; client.read_exact(&mut port_b).await?;
            (addr.to_string(), u16::from_be_bytes(port_b))
        }
        0x03 => { // Domain
            // Read domain length
            let mut len = [0u8; 1]; client.read_exact(&mut len).await?;
            // Read domain name
            let mut domain = vec![0u8; len[0] as usize];
            client.read_exact(&mut domain).await?;
            let host = String::from_utf8(domain)?;
            // Read 2 bytes for port (big-endian)
            let mut port_b = [0u8; 2]; client.read_exact(&mut port_b).await?;
            (host, u16::from_be_bytes(port_b))
        }
        _ => return Err(anyhow::anyhow!("Address type not supported")), // IPv6 not supported yet
    };

    println!("Connecting to {}:{} via Tor...", target.0, target.1);

    // Connect through Tor's SOCKS5
    let tor_stream = Socks5Stream::connect(TOR_SOCKS_ADDR, target).await?;
    let tor = tor_stream.into_inner();

    // Reply success to client
    let reply = [0x05, 0x00, 0x00, 0x01, 0,0,0,0, 0,0];
    client.write_all(&reply).await?;

    // Bidirectional copy
    let (mut cr, mut cw) = split(client);
    let (mut tr, mut tw) = split(tor);
    tokio::try_join!(copy(&mut cr, &mut tw), copy(&mut tr, &mut cw))?;

    Ok(())
}
