#![feature(generic_associated_types)]

mod findorad;
mod evm;
mod config;

fn main() {
    env_logger::init();

    // create evm rpc port.

    findorad::start_findorad();
}
