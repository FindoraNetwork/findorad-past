mod issue;
pub use issue::Issue;

mod transfer;
pub use transfer::Transfer;

mod define;
pub use define::Define;

mod delegate;
pub use delegate::*;

mod stake;
pub use stake::*;

mod undelegate;
pub use undelegate::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    Define(Define),
    Issue(Issue),
    Transfer(Transfer),
    Delegate(Delegate),
    Stake(Stake),
    Undelegate(Undelegate),
}
