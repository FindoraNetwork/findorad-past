mod issue;
pub use issue::Issue;

mod transfer;
pub use transfer::Transfer;

mod define;
pub use define::Define;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    Define(Define),
    Issue(Issue),
    Transfer(Transfer),
}
