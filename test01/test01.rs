
#![no_main]    // this file does not contain a main function

#[no_mangle]   // we do not want to mangle the symbol when exporting
pub extern fn add(a: i32, b: i32) -> i32 { a + b }
