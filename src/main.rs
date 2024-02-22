use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html!(<h1 class="text-dark-orange ">{"Hello, wolrd!"}</h1>)
}

fn main() {
    yew::Renderer::<App>::new().render();
}
