use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html!(<h1>{"Hello, wolrd!"}</h1>)
}

fn main() {
    yew::Renderer::<App>::new().render();
}
