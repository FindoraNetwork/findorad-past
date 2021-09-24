use clap::Clap;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short)]
    /// Findorad rpc address.
    server_address: String,

    #[clap(short, long)]
    /// Mnemonic path
    mnemonic_path: String,
}

impl Command {
    pub fn execute(&self) {

    }
}
