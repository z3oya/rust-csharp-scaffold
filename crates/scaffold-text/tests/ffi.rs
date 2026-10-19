use scaffold_text::ByteBuffer;

#[used]
#[allow(clippy::type_complexity)]
static KEEP_EXPORTS: (
    extern "C" fn() -> i32,
    unsafe extern "C" fn(*const u8, u32) -> *mut ByteBuffer,
    unsafe extern "C" fn(*mut ByteBuffer),
) = (
    scaffold_text::scaffold_text_abi_version,
    scaffold_text::scaffold_text_greet,
    scaffold_text::scaffold_text_free_byte_buffer,
);

unsafe extern "C" {
    fn scaffold_text_abi_version() -> i32;
    fn scaffold_text_greet(name: *const u8, name_len: u32) -> *mut ByteBuffer;
    fn scaffold_text_free_byte_buffer(buffer: *mut ByteBuffer);
}

fn take_greeting(ptr: *mut ByteBuffer) -> Option<String> {
    unsafe {
        if ptr.is_null() {
            return None;
        }
        let buffer = *ptr;
        assert!(buffer.capacity >= buffer.length);
        let bytes = std::slice::from_raw_parts(buffer.ptr, buffer.length as usize).to_vec();
        scaffold_text_free_byte_buffer(ptr);
        String::from_utf8(bytes).ok()
    }
}

#[test]
fn abi_version_is_current() {
    let version = unsafe { scaffold_text_abi_version() };
    assert_eq!(version, 1);
}

#[test]
fn greet_returns_text() {
    let name = "Ada";
    let ptr = unsafe { scaffold_text_greet(name.as_ptr(), name.len() as u32) };
    let text = take_greeting(ptr).expect("greet returned null");
    assert_eq!(text, "Hello, Ada, from the Rust backend!");
}

#[test]
fn greet_rejects_invalid_utf8() {
    let bytes: [u8; 2] = [0xFF, 0xFE];
    let ptr = unsafe { scaffold_text_greet(bytes.as_ptr(), bytes.len() as u32) };
    assert!(ptr.is_null());
}

#[test]
fn greet_handles_empty_name() {
    let ptr = unsafe { scaffold_text_greet(std::ptr::null(), 0) };
    let text = take_greeting(ptr).expect("greet returned null");
    assert_eq!(text, "Hello, , from the Rust backend!");
}
