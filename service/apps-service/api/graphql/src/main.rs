use actix_web::{dev::Service, main, App, HttpServer};
use graphql::api;
use infra_utils::{anyhow, ssl::ssl_init, trace::Trace, tracing};
#[main]
async fn main() -> anyhow::Result<()> {
    let _trace = Trace::init();
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
        let builder = ssl_init("pem/key.pem", "pem/cert.pem")?;
        server.bind_rustls(addr, builder)?
    } else {
        let server = server.bind(addr)?;
        server
    };
    server.run().await?;
    Ok(())
}
