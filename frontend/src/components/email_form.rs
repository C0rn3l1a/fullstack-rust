use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(EmailForm)]
pub fn email_form_component() -> Html {
    let email_value = use_state(|| String::from(""));
    
    let on_input = {
        // clone state into block to use it
        let email = email_value.clone();
        Callback::from(move |event: InputEvent| {
            // equivalent to to event.target in JS
            let target: HtmlInputElement = event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            // set extracted value in the state
            email.set(target.value());
        })
    };

    let on_click = {
        // clone state into block to use it
        let email = email_value.clone();
        Callback::from(move |_| {
            let greeting = format!("Send email to: {}", *email);
            // console log equivalent
            web_sys::console::log_1(&greeting.into());
        })
    };

    html! {
        <div class="flex items-center justify-center gap-4 " >
            <input type="email" placeholder="email" name="" id="" class="text-gray-900 px-4 py-2 rounded" oninput={on_input} value={email_value.to_string()} />
            <button onclick={on_click} class="bg-orange-500 px-4 py-2 rounded">{"Send!"}</button>
        </div>
    }
}