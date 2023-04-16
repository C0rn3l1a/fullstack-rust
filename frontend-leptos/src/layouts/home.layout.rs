use leptos::{component, view, IntoView, Scope};
use crate::components::header::*;
use crate::components::email_form::*;

#[component]
pub fn HomeLayout(cx: Scope) -> impl IntoView {
    view! {cx, 
        <div class="text-orange-50">
            <div class="absolute block -z-10 h-screen w-screen overflow-hidden " >
                <img src="/static/images/image.png" class="h-screen min-w-fit blur-xl scale-150"/>
            </div>
            <HeaderComponent />
            <main class="flex flex-col" >
                <section class="flex items-center justify-center gap-4 h-screen">
                    <EmailFormComponent />
                </section>
            </main>
        </div>
    }
}