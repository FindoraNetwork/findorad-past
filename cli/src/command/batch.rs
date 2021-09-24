use clap::Clap;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long, default_value = "")]
    /// Batch id
    id: String,
}
