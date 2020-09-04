#[no_mangle]
pub extern fn init() {
    println!("Zach has a small penis");
}

#[no_mangle]
pub extern fn run(input: &str) -> String {
    format!("input: {} so does nick", input)
}

#[no_mangle]
pub extern fn reply(input: &str) -> plugin_api::MyCoolMessage {
    plugin_api::MyCoolMessage {
        name: "penis",
        year: "42069",
        country: input
    }
}
