// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use bindings;
    use std::ptr;

    #[test]
    fn test_bd_init() {
        unsafe {
            bindings::bd_init(ptr::null_mut(), None, ptr::null_mut());
        }
    }
}
