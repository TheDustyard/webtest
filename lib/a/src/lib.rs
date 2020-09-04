#[no_mangle]
pub extern fn init() {
    println!("My cool plugin init message: a");
}

#[no_mangle]
pub extern fn run(input: &str) -> String {
    format!("input: {}", input)
}

#[no_mangle]
pub extern fn reply(input: &str) -> plugin_api::MyCoolMessage {
    plugin_api::MyCoolMessage {
        name: "jeff",
        year: "1998",
        country: input
    }
}
