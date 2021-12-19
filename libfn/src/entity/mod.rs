mod issue;
pub use issue::Issue;

mod transfer;
pub use transfer::Transfer;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    Issue(Issue),
    Transfer(Transfer),
}
