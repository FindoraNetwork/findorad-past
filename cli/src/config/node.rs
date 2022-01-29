use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub address: String,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            address: String::from("http://localhost:26657"),
        }
    }
}
