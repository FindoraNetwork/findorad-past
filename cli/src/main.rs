use clap::Clap;

pub mod command;

fn main() {
    let opts = command::Fn::parse();
    println!("{:?}", opts);
}
