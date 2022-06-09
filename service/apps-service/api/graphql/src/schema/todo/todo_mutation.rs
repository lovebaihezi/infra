use async_graphql::*;
use database::{
    todo_rpc::{todo_service_client::TodoServiceClient, Todo, TodoUpdate},
    todo_rpc::{DeleteQuery, TodoAdd},
    RPC_CONNECT,
};

use infra_utils::tracing;
use infra_utils::tracing::instrument;
use serde::{Deserialize, Serialize};
use tonic::Request;

#[derive(Debug)]
pub struct MutationTodo;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Empty;
scalar!(Empty);

#[Object]
impl MutationTodo {
    #[instrument(skip(changes))]
    async fn update_todos(&self, changes: Vec<Todo>) -> Result<Empty> {
        tracing::trace!("connect to RPC: {}", RPC_CONNECT);
        let mut client = TodoServiceClient::connect(RPC_CONNECT).await?;
        let req = Request::new(TodoUpdate { todos: changes });
        let _ = client.update_todo(req).await?;
        Ok(Default::default())
    }
    #[instrument(skip(new_todo))]
    async fn add_todos(&self, new_todo: Vec<Todo>) -> Result<Empty> {
        tracing::trace!("connect to RPC: {}", RPC_CONNECT);
        let mut client = TodoServiceClient::connect(RPC_CONNECT).await?;
        tracing::trace!("call rpc todo add");
        let req = Request::new(TodoAdd { todos: new_todo });
        let _ = client.add_todo(req).await?;
        Ok(Default::default())
    }
    #[instrument]
    async fn delete_todos(&self, delete: Vec<String>) -> Result<Empty> {
        tracing::trace!("connect to RPC: {}", RPC_CONNECT);
        let mut client = TodoServiceClient::connect(RPC_CONNECT).await?;
        tracing::trace!("call rpc todo add");
        let req = Request::new(DeleteQuery { ids: delete });
        let _ = client.delete_todo(req).await?;
        Ok(Default::default())
    }
}
