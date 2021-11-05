#![feature(generic_associated_types)]

mod command;
mod node;
mod entry;

use clap::Parser;

 pub fn main() {
    env_logger::init();
    let opts = command::Opts::parse();

    match opts.execute() {
        Ok(_) => {}
        Err(e) => {
            e.print(None);
            //             let mut app = command::Opts::into_app();
            //             app.print_help().unwrap();
            panic!();
        }
    }
}
