use std::error::Error;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement};
use yew::prelude::*;
use crate::components::toast::ToastContext;
use gloo_net::http::Request;

#[function_component(EmailForm)]
pub fn email_form_component() -> Html {
    let msg_ctx = use_context::<ToastContext>().unwrap();
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

    let on_submit = Callback::from(move |event: SubmitEvent| {
            // I want to get formdata here, including hidden inputs
            event.prevent_default();

            let form = event
                .target()
                .unwrap_throw()
                .dyn_into::<web_sys::HtmlFormElement>()
                .unwrap_throw();
            let form_data = web_sys::FormData::new_with_form(&form).unwrap_throw();
            let cf_turnstile_value = form_data.get("cf-turnstile-response");
            let email_value = form_data.get("email");
            let name_value = form_data.get("name");
            let phone_value = form_data.get("phone");
            let message_value = form_data.get("message");
            web_sys::console::log_1(&cf_turnstile_value);
            web_sys::console::log_1(&email_value);
            wasm_bindgen_futures::spawn_local(async move {
                if cf_turnstile_value.is_string() 
                    & email_value.is_string()
                    & name_value.is_string()
                    & phone_value.is_string()
                    & message_value.is_string()
                {
                    let cf_turnstile = cf_turnstile_value.as_string().unwrap();
                    let email = email_value.as_string().unwrap();
                    let name = name_value.as_string().unwrap();
                    let phone = phone_value.as_string().unwrap();
                    let message = message_value.as_string().unwrap();
                    // todo: turn `cf_turnstile_value` and `email_value` into strings
                    match send_email(cf_turnstile, email, name, phone, message).await {
                        Ok(_) => web_sys::console::log_1(&String::from("request success").into()),
                        Err(error) => web_sys::console::log_1(&error.to_string().into())
                    };
                }
            });
        });

    html! {
        <form class="flex flex-col items-strech justify-center w-full px-4 gap-6" method="POST" onsubmit={on_submit}>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="name"><span class="text-orange-400 material-symbols-outlined">{"emoji_people"}</span> {"Name"}</label>
                <input required={true} type="text" placeholder="name" name="name" id="name" class="w-full text-gray-900 px-4 py-2 rounded"/>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="email"><span class="text-orange-400 material-symbols-outlined">{"contact_mail"}</span> {"Email"}</label>
                <input required={true} type="email" placeholder="email" name="email" id="email" class="w-full text-gray-900 px-4 py-2 rounded"/>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="phone"><span class="text-orange-400 material-symbols-outlined">{"phone"}</span> {"Phone"}</label>
                <input required={true} type="tel" pattern="+[0-9]{12}" list="phone-example" minlength="9" maxlength="14" placeholder="phone" name="phone" id="phone" class="w-full text-gray-900 px-4 py-2 rounded"/>
                <datalist id="phone-example">
                    <option value="+611234567890"></option>
                </datalist>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="message"><span class="text-orange-400 material-symbols-outlined">{"message"}</span> {"Message"}</label>
                <textarea required={true} placeholder="message" name="message" id="message" class="w-full text-gray-900 px-4 py-2 rounded h-36 resize-none"/>
            </div>
            
            <div class="flex items-center justify-center gap-4">
                <button type="submit" class="flex items-center justify-center gap-2 bg-orange-400 px-4 py-2 rounded">
                    <span>{"Send"}</span>
                    <span class="material-symbols-outlined">{"send"}</span>
                </button>
            </div>

            <div class="flex items-center justify-center gap-4 mt-auto">
                // The following line controls and configures the Turnstile widget.
                <div class="cf-turnstile" data-sitekey="1x00000000000000000000AA" data-theme="dark"></div>
                // end.
            </div>
            <script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async={true} defer={true}></script>
        </form>
    }
}

#[derive(Deserialize, Serialize)]
pub struct SendEmailBody {
    cf_turnstile_token: String,
    email: String,
    name: String,
    phone: String,
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyResponse {
    message: String
}

pub async fn send_email(cf_turnstile_token: String, email: String, name: String, phone: String, message: String) -> Result<bool, Box<dyn Error>> {
    let message = SendEmailBody { cf_turnstile_token, email, name, phone, message };

    let response: VerifyResponse = Request::post("http://localhost:3000/verify").json(&message)?.send().await?.json().await?;

    web_sys::console::log_1(&response.message.into());

    Ok(true)
}