pub(crate) fn write_to_buffer(
    output: &String,
    buf: *mut cty::c_char,
    max_len: cty::c_int,
) -> cty::c_int {
    let src = output.as_bytes().as_ptr();
    let len = output.as_bytes().len();
    let len_c_int = len as cty::c_int;
    if len_c_int < max_len {
        unsafe {
            std::ptr::copy(src, buf as *mut u8, len);
            (*buf.add(len)) = 0;
        }
        len_c_int
    } else {
        println!("required length is {}", len_c_int);
        -1000
    }
}