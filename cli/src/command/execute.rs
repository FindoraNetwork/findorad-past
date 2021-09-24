use clap::Clap;
use ruc::*;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Findorad rpc address.
    id: String,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
