use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: AttrValue,
    pub alt: AttrValue,
}
#[function_component(Logo)]
pub fn logo(props: &Props) -> Html {
    html! {
          <img class="mx-auto h-10 w-auto" src={props.src.clone()} alt={props.alt.clone()}/>
    }
}