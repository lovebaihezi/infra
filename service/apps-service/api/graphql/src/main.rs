use actix_web::{dev::Service, main, App, HttpServer};
use graphql::api;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};
use tracing::instrument;

#[instrument]
async fn ssl_init(key_pem: &'static str, cert_pem: &'static str) -> std::io::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert = &mut BufReader::new(File::open(cert_pem)?);
    let key = &mut BufReader::new(File::open(key_pem)?);
    let cert_chain = certs(cert)?.into_iter().map(Certificate).collect();
    let mut key: Vec<_> = pkcs8_private_keys(key)?
        .into_iter()
        .map(PrivateKey)
        .collect();
    assert!(
        !key.is_empty(),
        "key should not be empty!Could not locale the pkcs8 key"
    );
    Ok(config.with_single_cert(cert_chain, key.remove(0)).unwrap())
}

#[main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();

    #[cfg(not(debug_assertions))]
    let (_, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never("log", "todo.log"));
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8888".into())
        .parse::<u16>()
        .unwrap();
    let addr = format!(
        "{}:{}",
        std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
        port
    );
    let server = HttpServer::new(|| {
        let app = App::new();
        #[cfg(debug_assertions)]
        let app = app.wrap_fn(|req, service| {
            let path = req.path();
            let address = req.peer_addr();
            let ip = address.map(|v| v.ip());
            let port = address.map(|v| v.port());
            let method = req.method().as_str();
            let socket_version = req.version();
            tracing::debug!(
                "<{:?}> [{:?}] {{{}}} |{:?}:{:?}|",
                socket_version,
                method,
                path,
                ip,
                port,
            );
            service.call(req)
        });
        app.configure(api::init)
    });
    let server = if cfg!(debug_assertions) {
        let builder = ssl_init("pem/key.pem", "pem/cert.pem").await?;
        server.bind_rustls(addr, builder)?
    } else {
        let server = server.bind(addr)?;
        server
    };
    server.run().await?;
    Ok(())
}
