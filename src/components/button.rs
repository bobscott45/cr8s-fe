use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub button_type: AttrValue,
}
#[function_component(Button)]
pub fn input(props: &Props) -> Html {
    html! {
        <div>
              <button type={props.button_type.clone()} class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{props.label.clone()}</button>
        </div>
    }
}