use async_graphql::*;
use serde::{Deserialize, Serialize};
mod todo_mutation;
mod todo_query;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    id: String,
    content: String,
    start_time: String,
    overdue_time: Option<String>,
    is_completed: bool,
}

scalar!(Todo);
pub use todo_mutation::MutationTodo;
pub use todo_query::QueryTodo;
pub type TodoSchema = Schema<QueryTodo, MutationTodo, EmptySubscription>;
