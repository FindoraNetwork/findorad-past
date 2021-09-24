use clap::Clap;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Findorad rpc address.
    id: String,
}

