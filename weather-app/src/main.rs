

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct TodoItem {
    userId: i64,
    id: i64,
    title: String,
    completed: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .json::<TodoItem>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}