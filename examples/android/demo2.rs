#[no_mangle]
pub extern "C" fn android_link_hack() {}

#[no_mangle]
pub extern fn print_something_from_rust() {
    println!("Ferris says hello!");
}

#[no_mangle]
pub extern fn get_a_name_from_rust() ->  &'static str {
    return "a random name";
}

#[no_mangle]
pub extern "system" fn Java_com_example_androidapp_JniShim2_getName() ->  &'static str {
    return get_a_name_from_rust()
}
