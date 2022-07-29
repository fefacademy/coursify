use std::{ffi::OsStr, fmt::Write};

use walkdir::WalkDir;

#[no_mangle]
pub extern "C" fn get_sections() -> *const *const u8 {
    let values = vec![
        "section-one\0".as_ptr(),
        "section-two\0".as_ptr(),
        "section-three\0".as_ptr(),
        "section-four\0".as_ptr(),
        "section-five\0".as_ptr(),
    ];
    let p = values.as_ptr();
    std::mem::forget(values);
    p
}

#[no_mangle]
pub extern "C" fn resolve_sections(dir: i32) -> *const u8 {
    format!("Hello {dir}\0").as_ptr()
}
