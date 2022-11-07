use primitive_types::H160;

mod ffi {
    extern "C" {
        pub fn get_proposer(height: i64, target: *mut u8);
    }
}

pub fn get_proposer(height: i64) -> H160 {
    let mut result = H160::default();

    let target_ptr = result.as_mut_ptr();

    unsafe {
        ffi::get_proposer(height, target_ptr);
    }

    result
}
