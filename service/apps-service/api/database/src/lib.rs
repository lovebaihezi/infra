pub mod todo;
pub mod todo_rpc {
    tonic::include_proto!("todo");
}

pub const RPC_ADDRESS: &str = env!("RPC_ADDRESS");
pub const RPC_CONNECT: &str = concat!("http://", env!("RPC_ADDRESS"));
pub type PgPool = sqlx::Pool<sqlx::Postgres>;
pub const FORMAT: &str = "%+";
