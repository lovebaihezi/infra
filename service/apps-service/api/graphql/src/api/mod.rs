use actix_web::{
    post,
    web::{Data, ServiceConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use tracing::instrument;

use crate::schema::{todo::TodoSchema, todo_schena};

#[instrument(skip(schema, request))]
#[post("/api")]
async fn api(schema: Data<TodoSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

pub fn init(ctx: &mut ServiceConfig) {
    let todo = todo_schena();
    ctx.app_data(Data::new(todo)).service(api);
}
