use async_graphql::{EmptySubscription, Schema};
pub mod todo;
use todo::TodoSchema;
pub fn todo_schena() -> TodoSchema {
    Schema::build(todo::QueryTodo, todo::MutationTodo, EmptySubscription).finish()
}
