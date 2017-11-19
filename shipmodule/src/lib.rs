

extern crate libc;

#[repr(C)]
pub struct GoString {
    chars: *const libc::c_char,
    len: i64,
}

impl Into<String> for GoString {
    fn into(self) -> String {
        let len = self.len as usize;
        unsafe {
            String::from_raw_parts(self.chars as *mut u8, len, len)
        }
    }
}

impl From<String> for GoString {
    fn from(s: String) -> Self {
        let chars = s.as_bytes().as_ptr() as *const libc::c_char;
        let len = s.len() as i64;
        GoString {
            chars: chars,
            len: len,
        }
    }
}

#[cfg(feature="link")]
#[link(name = "dhl")]
extern "C" {
    pub fn GetLabel(input: GoString);
}

#[no_mangle]
pub extern "C" fn rust_expose(input: *const libc::c_char) {
    println!("Rust: {:?}", input);
}

