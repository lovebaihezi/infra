use std::error::Error;
use std::str::FromStr;

use crate::todo_rpc::todo_service_server;
use crate::todo_rpc::Todo;
use crate::PgPool;
use crate::{todo_rpc, FORMAT};
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use tonic::{Code, Request, Response, Status};
use tracing::instrument;

type TimeStamp = DateTime<Utc>;

type ParseResult = Result<(Uuid, TimeStamp, Option<TimeStamp>), Status>;

fn take<T, R: Error>(t: Result<T, R>) -> Result<T, Status> {
    t.map_or_else(|e| Err(Status::unknown(e.to_string())), Ok)
}

#[instrument]
fn parse(todo: &Todo) -> ParseResult {
    tracing::debug!("parser id");
    let id = take(Uuid::from_str(todo.id.as_str()))?;
    tracing::debug!("parse start_time");
    let start_time = take(NaiveDateTime::parse_from_str(&todo.start_time, FORMAT))?;
    let start_time = TimeStamp::from_utc(start_time, Utc);
    tracing::debug!("parse overdue_time");
    let overdue_time = match &todo.overdue_time {
        Some(v) => {
            let overdue_time = take(NaiveDateTime::parse_from_str(v, FORMAT))?;
            let overdue_time = DateTime::<Utc>::from_utc(overdue_time, Utc);
            Some(overdue_time)
        }
        None => None,
    };
    tracing::debug!("parse finished");
    Ok((id, start_time, overdue_time))
}

pub struct TodoService {
    pool: PgPool,
}

impl TodoService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TodoData {
    id: sqlx::types::Uuid,
    content: String,
    start_time: DateTime<Utc>,
    overdue_time: Option<DateTime<Utc>>,
    is_completed: bool,
    index: i32,
}

impl TodoData {
    fn todo(self) -> Todo {
        Todo {
            id: self.id.to_string(),
            content: self.content,
            start_time: self.start_time.to_string(),
            overdue_time: self.overdue_time.map(|v| v.to_string()),
            is_completed: self.is_completed,
            index: self.index,
        }
    }
}

#[tonic::async_trait]
impl todo_service_server::TodoService for TodoService {
    #[instrument(skip_all())]
    async fn query_todo(
        &self,
        _: Request<todo_rpc::TodoQuery>,
    ) -> Result<Response<todo_rpc::QueryResult>, Status> {
        tracing::debug!("query all todos witch sqlx");
        let todos = sqlx::query_as!(
            TodoData,
            "SELECT id, content, start_time, overdue_time, is_completed, index FROM todo;"
        )
        .map(|v| v.todo())
        .fetch_all(&self.pool)
        .await;
        match todos {
            Ok(todos) => {
                let response = Response::new(todo_rpc::QueryResult { todos });
                Ok(response)
            }
            Err(err) => Err(Status::new(Code::Unknown, err.to_string())),
        }
    }

    #[instrument(skip_all())]
    async fn update_todo(
        &self,
        req: Request<todo_rpc::TodoUpdate>,
    ) -> Result<Response<todo_rpc::UpdateResult>, Status> {
        tracing::trace!("update todos");
        let todos = req.into_inner().todos;
        for todo in todos {
            let (id, start_time, overdue_time) = parse(&todo)?;
            take(sqlx::query!(
                r#"UPDATE todo SET content = $1 , start_time = $2, overdue_time = $3 , is_completed = $4 WHERE id = $5 ;"#,
                todo.content,
                start_time,
                overdue_time,
                todo.is_completed,
                id
            ).fetch_all(&self.pool).await)?;
        }
        Ok(Response::new(todo_rpc::UpdateResult {}))
    }

    #[instrument(skip_all())]
    async fn add_todo(
        &self,
        req: Request<todo_rpc::TodoAdd>,
    ) -> Result<Response<todo_rpc::AddResult>, Status> {
        tracing::trace!("add todo");
        let todos = req.into_inner().todos;
        for todo in todos {
            tracing::trace!("parse id, start_time, overdue_time from todo");
            let (id, start_time, overdue_time) = parse(&todo)?;
            tracing::trace!("insert into todo");
            take(sqlx::query!(
                "INSERT INTO todo (ID, content, start_time, overdue_time, is_completed, index) VALUES ($1, $2, $3, $4, $5, $6)",
                id,
                todo.content,
                start_time,
                overdue_time,
                todo.is_completed,
                todo.index
                ).fetch_all(&self.pool).await)?;
        }
        Ok(Response::new(todo_rpc::AddResult {}))
    }

    #[instrument(skip_all())]
    async fn delete_todo(
        &self,
        req: Request<todo_rpc::DeleteQuery>,
    ) -> Result<Response<todo_rpc::DeleteResult>, Status> {
        tracing::trace!("delete todo");
        let ids = req.into_inner().ids;
        for id in ids {
            let id = take(Uuid::from_str(&id))?;
            take(
                sqlx::query!("DELETE FROM todo WHERE id = $1;", id)
                    .fetch_all(&self.pool)
                    .await,
            )?;
        }
        Ok(Response::new(todo_rpc::DeleteResult {}))
    }
}

#[cfg(test)]
mod todo_tests {
    use chrono::NaiveDateTime;

    use crate::FORMAT;

    #[test]
    fn time_parse() {
        let time = "2022-03-26T16:24:40.123Z";
        NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        NaiveDateTime::parse_from_str(time, FORMAT).unwrap();
    }
}
