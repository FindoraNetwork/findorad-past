use primitive_types::H160;

mod ffi {
    extern "C" {
        pub fn set_rewards(address: *const u8, validator: *const u8, reward: u64);
    }
}

pub fn set_rewards(address: H160, validator: H160, reward: u64) {
    let address = address.as_ptr();
    let validator = validator.as_ptr();

    unsafe { ffi::set_rewards(address, validator, reward) }
}

