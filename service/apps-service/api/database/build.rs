use std::{collections::VecDeque, path::PathBuf};

use tokio::fs::{read_dir, ReadDir};

async fn find_all(proto_dir: ReadDir) -> std::io::Result<Vec<PathBuf>> {
    let mut queue = VecDeque::with_capacity(512);
    let mut proto_paths = Vec::with_capacity(512);
    queue.push_back(proto_dir);
    while !queue.is_empty() {
        let mut top = queue.pop_front().unwrap();
        while let Some(entry) = top.next_entry().await? {
            let file_type = entry.file_type().await?;
            if file_type.is_dir() {
                queue.push_back(read_dir(entry.path()).await?);
            } else {
                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .expect("can not convert file name into &str!");
                if file_name.ends_with(".proto") && file_name.len() > 6 {
                    proto_paths.push(entry.path());
                }
            }
        }
    }
    Ok(proto_paths)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let attribute = r#"
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, async_graphql::InputObject)]
#[serde(rename_all = "camelCase")]"#;
    let compiler = tonic_build::configure()
        .type_attribute("todo.Todo", attribute)
        .type_attribute("todo.TodoUpdate", attribute)
        .type_attribute("todo.TodoAdd", attribute)
        .type_attribute("todo.QueryResult", attribute);
    let proto_paths = find_all(
        read_dir("../../../../proto")
            .await
            .expect("can't find specific dir"),
    )
    .await?;
    compiler.compile(proto_paths.as_slice(), &["../../../../proto"])?;
    Ok(())
}
