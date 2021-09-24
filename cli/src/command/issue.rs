use clap::Clap;
use ruc::*;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long, default_value = "")]
    /// Batch id
    id: String,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
