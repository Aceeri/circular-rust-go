
extern crate shipmodule;

use shipmodule::GoString;

fn main() {
    unsafe {
        shipmodule::GetLabel(GoString::from("Rust->Go ffi :D".to_string()));
    }
}
