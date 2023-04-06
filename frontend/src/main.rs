#[macro_use]
extern crate dotenv_codegen;

use yew::prelude::*;
mod components;

use components::{header::Header, email_form::EmailForm, toast::{Toast, ToastProvider}};

#[function_component]
fn App() -> Html {
    html! {
        <ToastProvider>
            <div class="text-orange-50">
                <div class="absolute block -z-10 h-screen w-screen overflow-hidden " >
                    <img src="https://i.redd.it/52f61nfzmwl51.jpg" class="h-screen min-w-fit blur-xl scale-150"/>
                </div>
                <Header />
                <main class="flex flex-col" >
                    <section class="flex items-center justify-center gap-4 h-screen">
                        <EmailForm />
                    </section>
                </main>
                <Toast />
            </div>
        </ToastProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}