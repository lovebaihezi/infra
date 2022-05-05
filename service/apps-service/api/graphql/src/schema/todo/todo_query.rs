use async_graphql::*;
use database::{
    todo_rpc::{todo_service_client::TodoServiceClient, QueryResult, TodoQuery},
    RPC_CONNECT,
};
use tonic::Request;
use tracing::instrument;

#[derive(Debug)]
pub struct QueryTodo;

#[Object]
impl QueryTodo {
    #[instrument]
    async fn todos(&self) -> Result<String> {
        tracing::debug!("connect to RPC: {}", RPC_CONNECT);
        let mut client = TodoServiceClient::connect(RPC_CONNECT).await?;
        tracing::debug!("call rpc function: query todo");
        let res = client.query_todo(Request::new(TodoQuery {})).await?;
        tracing::debug!("rpc exec finished!");
        let QueryResult { todos } = res.into_inner();
        Ok(serde_json::to_string(&todos)?)
    }
}
