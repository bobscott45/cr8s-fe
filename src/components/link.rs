use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub href: AttrValue,
}
#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    html! {
        <a href={props.href.clone()} class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">{props.label.clone()}</a>
    }
}