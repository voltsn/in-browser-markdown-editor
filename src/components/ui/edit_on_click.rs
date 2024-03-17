use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_click_away;

#[derive(Properties, PartialEq)]
pub struct EditOnclickProps {
    #[prop_or("Untitled Document".to_string())]
    pub title: String,
    #[prop_or(Callback::noop())]
    pub on_input: Callback<Option<String>>,
}

#[function_component]
pub fn EditOnClick(props: &EditOnclickProps) -> Html {
    let title = use_state(|| props.title.clone());
    let is_editable = use_state(|| false);
    let element = use_node_ref();

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            title.set(value)
        })
    };

    let on_click = {
        let is_editable = is_editable.clone();
        Callback::from(move |_| is_editable.set(true))
    };

    {
        let is_editable = is_editable.clone();
        use_click_away(element.clone(), move |_| is_editable.set(false))
    };

    let content = {
        match *is_editable.clone() {
            false => html!(<div onclick={on_click}>{(*title).clone()}</div>),
            true => {
                html!(<input {oninput} type="text" value={(*title).clone()}/>)
            }
        }
    };
    html!(
    <div ref={element}>
        { content }
    </div>
    )
}
