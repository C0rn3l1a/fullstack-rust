use yew::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
    
    html! {
        <header class="absolute inset-x-0 top-0 z-50">
            <nav class="flex items-center justify-between p-6 lg:px-8" aria-label="Global">
                <div class="flex items-center gap-4">
                    <img src="https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/poi.svg" class="h-8 w-auto" alt="Logo"/>
                    <span>{"Sample Website"}</span>
                </div>
                <div class="flex items-center gap-4">
                    <button class="bg-orange-400 px-4 py-2 rounded">{"Explore"}</button>
                </div>
            </nav>
        </header>
    }
}