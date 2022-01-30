use primitive_types::H160;

mod ffi {
    extern "C" {
        pub fn get_validator_list_len(height: i64) -> usize;

        pub fn get_validator_list_item(height: i64, index: usize, res: *mut u8);
    }
}

pub fn get_validator_list(height: i64) -> Vec<H160> {
    let mut res = Vec::new();

    let length = unsafe { ffi::get_validator_list_len(height) };

    for i in 0..length {
        let mut res_delegator = H160::default();

        let delegator = res_delegator.as_mut_ptr();

        unsafe { ffi::get_validator_list_item(height, i, delegator); }

        res.push(res_delegator);
    }

    res
}

