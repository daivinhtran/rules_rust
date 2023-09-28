//! A simple hello world example that can be called from C

#[no_mangle]
/// Print "Hello Rust!"
pub extern "C" fn print_c_hello_rust() {
    println!("Hello Rust!");
}
