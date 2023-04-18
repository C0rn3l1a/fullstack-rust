use std::time::Duration;

use gloo_net::http::Request;
use leptos::{component, view, IntoView, Scope, create_signal, create_local_resource, ReadSignal, WriteSignal, SignalGet, SignalSet, set_timeout};

use anyhow::Result;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{SubmitEvent, HtmlFormElement, FormData, console};
use api_interfaces::routes::contact::{ContactResponse, ContactBody};

#[derive(Clone,PartialEq,Debug)]
enum ComponentState {
    Initial,
    Loading,
    Error,
    Finished
}

#[component]
pub fn EmailFormComponent(cx: Scope) -> impl IntoView {
    let (form_value, set_form_value): (ReadSignal<Option<ContactBody>>, WriteSignal<Option<ContactBody>>) = create_signal(cx, None);
    let (state_value, set_state_value) = create_signal(cx, ComponentState::Initial);

    let data_sitekey: &str = dotenv!("DATA_SITEKEY");

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
        let message = form_data.get("message").as_string().unwrap();
        let cf_turnstile_token = form_data.get("cf-turnstile-response").as_string().unwrap();

        set_form_value.set(Some(ContactBody{
            email,
            phone,
            name,
            message,
            cf_turnstile_token
        }));
        set_state_value.set(ComponentState::Loading);
    };

    
    let response = create_local_resource(
        cx, 
        move ||{form_value.get()},
        send_email);

    let response_view = move || {
        response.with(cx, |data| {
            match data {
                Ok(res) => {
                    if res.len() > 0 {
                        set_state_value.set(ComponentState::Finished);
                        set_timeout(move || {set_form_value.set(None)}, Duration::from_millis(4000));
                        view!{cx, <>
                            <div class="flex flex-col items-start justify-center gap-2" >
                                <h3 class="bg-green-500 text-lg text-white py-2 px-4 rounded drop-shadow-md">{res}</h3>
                            </div>
                        </>}
                    } else {
                        view!{cx, <></>}
                    }
                },
                Err(error) => {
                    set_state_value.set(ComponentState::Error);
                    view!{cx, <>
                        <div class="flex flex-col items-start justify-center gap-2" >
                            <h3 class="bg-red-500 text-lg text-white py-2 px-4 rounded drop-shadow-md">{error.to_string()}</h3>
                        </div>
                    </>}
                }
            }
        })
    };

    view! { cx,
        <form class="flex flex-col items-strech justify-center w-full md:max-w-4xl px-4 gap-6" method="POST" on:submit=move|event| form_submit(event)>
            {move || {
                {response_view}
            }}
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="name"><span class="text-orange-400 material-symbols-outlined">"emoji_people"</span> "Name"</label>
                <input required={true} disabled={move || {state_value.get() != ComponentState::Initial}} type="text" placeholder="name" name="name" id="name" class="w-full text-gray-900 px-4 py-2 rounded"/>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="email"><span class="text-orange-400 material-symbols-outlined">"contact_mail"</span> "Email"</label>
                <input required={true} disabled={move || {state_value.get() != ComponentState::Initial}} type="email" placeholder="email" name="email" id="email" class="w-full text-gray-900 px-4 py-2 rounded"/>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="phone"><span class="text-orange-400 material-symbols-outlined">"phone"</span> "Phone"</label>
                <input required={true} disabled={move || {state_value.get() != ComponentState::Initial}} type="tel" list="phone-example" minlength="9" maxlength="14" placeholder="phone" name="phone" id="phone" class="w-full text-gray-900 px-4 py-2 rounded"/>
                <datalist id="phone-example">
                    <option value="+611234567890"></option>
                </datalist>
            </div>
            <div class="flex flex-col items-start justify-center gap-2" >
                <label class="flex items-center justify-center gap-2" for="message"><span class="text-orange-400 material-symbols-outlined">"message"</span> "Message"</label>
                <textarea required={true} disabled={move || {state_value.get() != ComponentState::Initial}} placeholder="message" name="message" id="message" class="w-full text-gray-900 px-4 py-2 rounded h-36 resize-none"/>
            </div>
            
            <div class="flex items-center justify-center gap-4">
                <button type="submit" disabled={move || {state_value.get() != ComponentState::Initial }} class="flex items-center justify-center gap-2 bg-orange-400 px-4 py-2 rounded disabled:bg-gray-400 disabled:cursor-not-allowed">
                    <span>"Send"</span>
                    {move || match state_value.get() {
                        ComponentState::Loading => view!{cx, <span class="material-symbols-outlined animate-spin">"autorenew"</span> },
                        _ => view!{cx, <span class="material-symbols-outlined">"send"</span> },
                    }}
                    
                </button>
            </div>

            <div class="flex items-center justify-center gap-4 mt-auto">
                // The following line controls and configures the Turnstile widget.
                <div class="cf-turnstile" data-sitekey={data_sitekey} data-theme="dark"></div>
                // end.
            </div>
            <script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async=true defer=true></script>
        </form>
    }
}

pub async fn send_email(message: Option<ContactBody>) -> Result<String> {
    if let Some(message) = message {
        let response: ContactResponse = Request::post("api/contact").json(&message)?.send().await?.json().await?;
        return Ok(response.message);
    } else {
        return Ok("".to_string());
    }
}