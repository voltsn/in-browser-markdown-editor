use yew::prelude::*;

mod components;

pub use components::ui::Button;
pub use components::ui::EditOnClick;

#[function_component]
pub fn App() -> Html {
    let icon = html!(<img src="icons/icon-save.svg"  alt="save icon"/>);
    html!(
        <>
        <Button title={"save changes".to_string()} {icon} />
        <Button title={"save document".to_string()} />
        <EditOnClick />
        </>)
}

fn main() {
    yew::Renderer::<App>::new().render();
}
