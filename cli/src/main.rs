use clap::Clap;

pub mod command;

fn main() {
    let opts = command::Opts::parse();
    opts.execute();
    println!("{:?}", opts);
}
