# RUST TOR PROXY

> **MI primer proyecto en Rust**: un proxy SOCKS5 local que enruta el tráfico a través de Tor de manera sencilla y segura.

---

## 📋 Índice

1. [Descripción](#descripción)
2. [Características](#características)
3. [Requisitos](#requisitos)
4. [Instalación](#instalación)
5. [Uso](#uso)
6. [Cómo funciona](#cómo-funciona)
7. [Estructura del proyecto](#estructura-del-proyecto)
8. [Contribuciones](#contribuciones)
9. [Licencia](#licencia)

---

## 📝 Descripción

Este repositorio contiene un **proxy SOCKS5** implementado en **Rust** que redirige todas tus conexiones TCP a través de la red de **Tor**, garantizando anonimato y privacidad.

Está diseñado pensando en la simplicidad y el rendimiento: aprovecha el poder de **Tokio** para I/O asíncrono y `tokio-socks` para conectar con Tor.

---

## 🔥 Características

- 🚀 **Asíncrono**: gracias a Tokio, maneja múltiples conexiones concurrentes sin bloqueo.
- 🔒 **Privacidad**: todo el tráfico pasa por la red Tor.
- 🔧 **Ligero**: sin dependencias complejas, solo Tokio y Tokio-Socks.
- 🛠️ **Configuración mínima**: funciona con los valores por defecto de Tor.
- 📈 **Logs sencillos**: registro de conexiones y errores en consola.

---

## 📦 Requisitos

- Rust **1.60+** (incluye `cargo`)
- Tor instalado y disponible en el puerto `9050` (o ajusta la configuración)
- Conexión a Internet

---

## 🔧 Instalación

1. **Clona el repositorio**
   ```bash
   git clone https://github.com/AsierCL/Rust-TORProxy
   cd Rust-TORProxy
   ```

2. **Compila el proyecto**
   ```bash
   cargo build --release
   ```

3. **Asegúrate de que Tor esté arrancado**
   - Por defecto, Tor debe escuchar en `127.0.0.1:9050`.
   - Si no está instalado, en Debian/Ubuntu:
     ```bash
     sudo apt update && sudo apt install tor
     sudo systemctl start tor
     ```

---

## ▶️ Uso

1. **Inicia el proxy**
    ```bash
    RUST_LOG=info ./target/release/TorRouter
    ```

2. **Configura tu aplicación o navegador** para usar SOCKS5 en `127.0.0.1:12345`.

3. **¡Navega anónimamente!** Todo tu tráfico TCP se enviará a través de Tor.

---

## ⚙️ Cómo funciona

1. **Arranca Tor** (se asume ya corriendo en `9050`).
2. El programa crea un **listener** en `127.0.0.1:12345`.
3. Cuando llega una conexión cliente:
   - Realiza un **handshake SOCKS5** sin autenticación.
   - Lee la petición `CONNECT` (IPv4 o dominio).
   - Conecta al destino a través de Tor usando `tokio-socks`.
   - Envía respuesta de éxito al cliente.
   - **Copiado bidireccional** de datos entre cliente y Tor.

Este patrón (split + copy) es el típico para proxies eficientes en Tokio.

---

## 📂 Estructura del proyecto

```text
rust-tor-proxy/
├── src/
│   └── main.rs       # Código principal del proxy
├── Cargo.toml        # Metadatos y dependencias
└── README.md         # Este archivo
```

---

## 🤝 Contribuciones

¡Bienvenidas! Si encuentras fallos o quieres proponer mejoras:

1. Abre un _issue_.
2. Crea un _fork_ y un _pull request_.

Por favor, sigue las [contribuciones estándar de GitHub](https://docs.github.com/es/github/collaborating-with-issues-and-pull-requests).

---

## 📄 Licencia

Este proyecto se distribuye bajo la licencia **MIT**. Consulta el archivo [LICENSE](LICENSE) para más detalles.

---

> **¡Gracias por usar `Rust-TORRouter`!** 👏

