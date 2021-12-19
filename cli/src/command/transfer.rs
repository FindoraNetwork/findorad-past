use clap::{Parser, ValueHint};

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Send tokens from the wallet to a specific address directly
    Send(Send),
    /// Save this sending request for further bach sending them together
    Save(Save),
    /// Batch sending the saved requests
    Batch(Batch),
    /// Show a list of saved batch process names or specific one for detail information
    Show(Show),
}

#[derive(Parser, Debug)]
struct Send {
    /// To specific a file path to the Findora wallet which contains base64-formatted XfrPrivateKey
    /// This option cannot be used with --from-secret-string
    #[clap(group = "from")]
    #[clap(short = 'f', long, value_name = "PATH", forbid_empty_values = true, value_hint = ValueHint::FilePath)]
    from_secret_file: Option<std::path::PathBuf>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted XfrPrivateKey
    /// This option cannot be used with --from-secret-file
    #[clap(group = "from")]
    #[clap(short = 's', long, value_name = "STRING", forbid_empty_values = true)]
    from_secret_string: Option<String>,
    /// Amount of FRA tokens to send
    #[clap(short, long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address to send which is a base64-formated XfrPublicKey
    #[clap(forbid_empty_values = true)]
    to_address: String,
    /// Make the amount confidential in the transaction
    #[clap(short = 'A', long)]
    confidential_amount: bool,
    /// Make the asset code confidential in the transaction
    #[clap(short = 'T', long)]
    confidential_asset: bool,
}

#[derive(Parser, Debug)]
struct Save {
    /// Name of the batch process for identifying in the batch command
    /// Save with the same batch name will appending the new request
    #[clap(short, long, required = true, forbid_empty_values = true)]
    batch_name: String,
    #[clap(flatten)]
    request: Send,
}

#[derive(Parser, Debug)]
struct Batch {
    /// Name of the batch process will be executing
    #[clap(forbid_empty_values = true)]
    batch_name: String,
}

#[derive(Parser, Debug)]
struct Show {
    /// Name of the batch process to show the request information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    batch_name: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.subcmd {
            SubCommand::Send(_send) => {}
            SubCommand::Save(_save) => {}
            SubCommand::Batch(_batch) => {}
            SubCommand::Show(_show) => {}
        }
        Ok(())
    }
}
