pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod entry;
pub(crate) mod utils;

use clap::Parser;

fn main() {
    env_logger::init();
    let opts = command::Opts::parse();

    match opts.execute() {
        Ok(_) => {}
        Err(_e) => {
            //             let mut app = command::Opts::into_app();
            //             app.print_help().unwrap();
            panic!();
        }
    }
}
