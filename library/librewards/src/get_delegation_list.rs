use primitive_types::H160;

mod ffi {
    extern "C" {
        pub fn get_delegation_list_len(validator: *const u8, height: i64) -> usize;

        pub fn get_delegation_list_item(
            validator: *const u8,
            height: i64,
            index: usize,
            delegator: *mut u8,
        );
    }
}

pub fn get_delegation_list(validator: H160, height: i64) -> Vec<H160> {
    let mut res = Vec::new();

    let validator = validator.as_ptr();

    let length = unsafe { ffi::get_delegation_list_len(validator, height) };

    for i in 0..length {
        let mut res_delegator = H160::default();

        let delegator = res_delegator.as_mut_ptr();

        unsafe { ffi::get_delegation_list_item(validator, height, i, delegator); }

        res.push(res_delegator);
    }

    res
}
