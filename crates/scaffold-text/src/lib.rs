use std::panic::{AssertUnwindSafe, catch_unwind};

pub struct Greeter;

impl Greeter {
    pub fn greet(name: &str) -> String {
        format!("Hello, {name}, from the Rust backend!")
    }
}

pub const OK: i32 = 0;
pub const ERR_INVALID_HANDLE: i32 = -1;
pub const ERR_BAD_ARGUMENT: i32 = -2;
pub const ERR_INTERNAL: i32 = -3;
pub const ABI_VERSION: i32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ByteBuffer {
    pub ptr: *mut u8,
    pub length: i32,
    pub capacity: i32,
}

impl ByteBuffer {
    fn from_vec(vec: Vec<u8>) -> Self {
        let mut vec = std::mem::ManuallyDrop::new(vec);
        if vec.capacity() == 0 {
            vec = std::mem::ManuallyDrop::new(Vec::with_capacity(1));
        }
        let length = vec.len() as i32;
        let capacity = vec.capacity() as i32;
        let ptr = vec.as_mut_ptr();
        Self {
            ptr,
            length,
            capacity,
        }
    }

    fn destroy_into_vec(self) -> Vec<u8> {
        if self.ptr.is_null() {
            return Vec::new();
        }
        unsafe { Vec::from_raw_parts(self.ptr, self.length as usize, self.capacity as usize) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_text_abi_version() -> i32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scaffold_text_greet(name: *const u8, name_len: u32) -> *mut ByteBuffer {
    let f = || {
        let bytes: &[u8] = if name.is_null() || name_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(name, name_len as usize) }
        };
        let name = std::str::from_utf8(bytes).ok()?;
        let text = Greeter::greet(name);
        let buffer = ByteBuffer::from_vec(text.into_bytes());
        Some(Box::into_raw(Box::new(buffer)))
    };
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Some(ptr)) => ptr,
        _ => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scaffold_text_free_byte_buffer(buffer: *mut ByteBuffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let boxed = Box::from_raw(buffer);
        drop(boxed.destroy_into_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(Greeter::greet("Ada"), "Hello, Ada, from the Rust backend!");
    }
}
