use database::{todo::TodoService, todo_rpc, RPC_ADDRESS};
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .pretty()
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    #[cfg(not(debug_assertions))]
    let (_, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never("log", "todo.rpc.log"));

    let pool = PgPoolOptions::new()
        .connect(env!("POSTGRES_POOL_URL"))
        .await
        .unwrap();
    let todo = TodoService::new(pool.clone());
    tracing::debug!("rpc server listen on {}", RPC_ADDRESS);
    Server::builder()
        .add_service(todo_rpc::todo_service_server::TodoServiceServer::new(todo))
        .serve(RPC_ADDRESS.parse().unwrap())
        .await
        .unwrap();
    Ok(())
}
