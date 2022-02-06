use primitive_types::H160;

mod ffi {
    extern "C" {
        pub fn get_delegation_amount(address: *const u8, validator: *const u8, height: i64) -> u64;
    }
}

pub fn get_delegation_amount(address: H160, validator: H160, height: i64) -> u64 {
    let address = address.as_ptr();
    let validator = validator.as_ptr();

    unsafe { ffi::get_delegation_amount(address, validator, height) }
}
