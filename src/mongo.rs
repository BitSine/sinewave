use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub prefix: Option<String>,
    pub id: u64,
    pub log_chnl_id: Option<u64>,
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
