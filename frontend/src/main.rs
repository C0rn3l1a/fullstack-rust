use leptos::*;

mod layouts;
mod components;

use layouts::home::*;

#[macro_use]
extern crate dotenv_codegen;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <HomeLayout/>
    })
}