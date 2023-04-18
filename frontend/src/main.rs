use leptos::*;
use dotenv::dotenv;

mod layouts;
mod components;

use layouts::home::*;

pub fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <HomeLayout/>
    })
}