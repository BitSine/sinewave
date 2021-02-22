use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub prefix: String,
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    warns: Vec<Warns>,
}

#[derive(Serialize, Deserialize)]
pub struct Warns {
    id: String,
    reason: String,
}
