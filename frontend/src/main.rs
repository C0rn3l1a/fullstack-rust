use anyhow::Result;
use leptos::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{Event, HtmlInputElement, console, HtmlFormElement, FormData, SubmitEvent};
use reqwasm::{http::Request};

mod layouts;
mod components;

use layouts::home::*;

#[component]
pub fn SimpleCounter(
    cx: Scope,
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);
    let (input_value, set_input_value) = create_signal(cx, String::from(""));

    let callback = move|event: Event| {
        let target = event.target()
            .unwrap_throw()
            .dyn_into::<HtmlInputElement>()
            .unwrap_throw();
        let value = target.value();
        set_input_value.set(value.clone());
        console::log_1(&value.into());
    };

    let form_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let target = event.target()
            .unwrap_throw()
            .dyn_into::<HtmlFormElement>()
            .unwrap_throw();
        
        let form_data = FormData::new_with_form(&target).unwrap_throw();

        let email = form_data.get("email").as_string().unwrap();
        let phone = form_data.get("phone").as_string().unwrap();
        let name = form_data.get("name").as_string().unwrap();
        console::log_1(&format!("email: {email}").into());
        console::log_1(&format!("phone: {phone}").into());
        console::log_1(&format!("name: {name}").into());
    };

    let once = create_local_resource(
        cx, 
        || (), 
        |_| async move { 
            match load_data().await {
                Ok(himsg) => himsg,
                Err(error) => error.to_string()
            } 
        });

    let loading = once.loading();

    view! { cx,
        <div style="display: flex; flex-direction: column; gap: 2rem">
            <div style="display: flex; gap: 2rem; padding: 1rem; border: solid 1px black;">
                <button on:click=move |_| set_value.set(0)>"Clear"</button>
                <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
            </div>
            <div style="display: flex; flex-direction: column; gap: 2rem; padding: 1rem; border: solid 1px black;">
                <input type="text" on:input=move |event| callback(event)/>
                <p>{input_value}</p>
            </div>
            <form on:submit=move|event| form_submit(event) style="display: flex; flex-direction: column; gap: 2rem; padding: 1rem; border: solid 1px black;">
                <input type="email" name="email" id="email" placeholder="email"/>
                <input type="phone"  name="phone" id="phone" placeholder="phone"/>
                <input type="text"  name="name" id="name" placeholder="name"/>
                <button type="submit">"Submit"</button>
            </form>
            <div style="display: flex; flex-direction: column; gap: 2rem; padding: 1rem; border: solid 1px black;">
                {move || if loading.get() {
                    view! {cx, <>
                        <span style="color:red">"Loading"</span>
                    </>}
                } else {
                    view! {cx, <>
                        <span style="color:green">"Not Loading"</span>
                        <span style="color:green">{once.read(cx)}</span>
                    </>}
                }}
                <p>{input_value}</p>
            </div>
        </div>
    }
}

#[derive(Serialize, Deserialize)]
struct HiResponse {
    message: String
}

async fn load_data() -> Result<String> {
    let response: HiResponse = Request::get("/api/hi").send().await?.json().await?;

    Ok(response.message)
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <HomeLayout/>
    })
}