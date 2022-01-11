#![feature(generic_associated_types)]

mod config;
mod evm;
mod findorad;

fn main() {
    env_logger::init();

    // create evm rpc port.

    findorad::start_findorad();
}
