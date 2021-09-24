use clap::Clap;
use ruc::*;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long, default_value = "http://localhost:25567")]
    /// Findorad rpc address.
    server_address: String,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        // Ok(())
        Err(eg!())
    }
}
