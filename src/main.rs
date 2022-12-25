use std::io::{Error, ErrorKind};
use std::str::FromStr;

use serde::Serialize;
use warp::{get, Filter};

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

/*
  See Ch. 10.2 in Rust Programming Language for use of Result<impl>
  https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits

*/
async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of Question".to_string(),
        Some(vec!["AMA".to_string()]),
    );

    Ok(warp::reply::json(&question))
}
