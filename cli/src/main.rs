use clap::Parser;

pub mod command;
pub mod config;
pub mod entry;
pub mod utils;

fn main() {
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
