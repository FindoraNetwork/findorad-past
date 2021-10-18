use clap::Parser;

pub mod command;
pub mod config;
pub mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    let opts = command::Opts::parse();

    match opts.execute().await {
        Ok(_) => {}
        Err(e) => {
            e.print(None);
            //             let mut app = command::Opts::into_app();
            //             app.print_help().unwrap();
            panic!();
        }
    }
}
