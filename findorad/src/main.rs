#![feature(generic_associated_types)]

mod config;
mod evm;
mod findorad;
fn main() {
    env_logger::init();

    findorad::start_findorad();
}
