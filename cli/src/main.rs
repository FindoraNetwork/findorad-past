use clap::{Clap, IntoApp};

pub mod command;
pub mod config;

fn main() {
    let opts = command::Opts::parse();

    match opts.execute() {
        Ok(_) => {}
        Err(e) => {
            e.print(None);

            let mut app = command::Opts::into_app();
            app.print_help().unwrap();

            panic!();
        }
    }
}
