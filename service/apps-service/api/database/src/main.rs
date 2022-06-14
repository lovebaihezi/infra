use database::{todo::TodoService, todo_rpc, RPC_ADDRESS};
use infra_utils::{anyhow, tokio, trace::Trace, tracing};
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _trace = Trace::init();
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
