use gloo_timers::callback::Timeout;
use std::{rc::Rc};
use yew::prelude::*;

#[function_component(Toast)]
pub fn toast_component() -> Html {
    let msg_ctx = use_context::<ToastContext>().unwrap();

    { // Set a timeout to clear the toast message after 4 seconds. TODO: configurable time
        let msg_ctx = msg_ctx.clone();
        let timeout = Timeout::new(4000, move || {msg_ctx.dispatch(String::from(""))});
        timeout.forget();
    }

    let message = msg_ctx.inner.to_owned();

    html! {
        if message.len() > 0 {
            <div class="animate-bounce fixed fixed top-28 left-0 right-0 flex items-center justify-center" >
                <p class="w-10/12 max-w-xl px-8 py-6 rounded bg-pink-600 shadow-md ">{message}</p>
            </div>
        }   
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub inner: String,
}

impl Reducible for Message {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { inner: action }.into()
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ToastProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub type ToastContext = UseReducerHandle<Message>;

#[function_component]
pub fn ToastProvider(props: &ToastProviderProps) -> Html {
    let msg = use_reducer(|| Message {
        inner: "".to_string(),
    });

    html! {
        <ContextProvider<ToastContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ToastContext>>
    }
}