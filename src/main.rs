use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Data {
    name: String,
    created_at: DateTime<Utc>,
}

fn main() {
    let d = Data {
        name: "n1".into(),
        created_at: Utc::now(),
    };
    dbg!(&d);

    let s = serde_json::to_string(&d).unwrap();
    println!("{}", s); // {"name":"n1","created_at":"2022-06-02T07:13:50.253712230Z"}
}
