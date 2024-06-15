use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Page {
    pub data: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Inputs {
    pub pages: Vec<Page>,
    pub merkle_root: Vec<u8>,
}
