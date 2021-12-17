use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub address: String,
    pub home: PathBuf,
}

impl Default for Node {
    fn default() -> Node {
        // must get "home"
        let mut home = home::home_dir().unwrap();
        home.push(".findora");
        home.push("fn");

        Node {
            address: String::from("http://localhost:25576"),
            home,
        }
    }
}
