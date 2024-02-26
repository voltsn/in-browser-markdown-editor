use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub title: String,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub icon: Html,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html!(<button class="btn-primary" onclick={props.on_click.clone()}>{props.icon.clone()} {props.title.clone()}</button>)
}
