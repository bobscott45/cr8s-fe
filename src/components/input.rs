use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
    pub required: bool,
    pub autocomplete: AttrValue,
    pub prompt:AttrValue,
}
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    html! {
   <div>
        <div class="flex items-center justify-between">
            <label for={props.name.clone()} class="block text-sm font-medium leading-6 text-gray-900">{props.label.clone()}</label>
            if props.prompt.len() > 0 {
                <div class={"text-sm"}>
                  <a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500">{props.prompt.clone()}</a>
                </div>
            }
        </div>
        <div class="mt-2">
            <input id={props.name.clone()} name={props.name.clone()}  type={props.input_type.clone()} required=true autocomplete={props.autocomplete.clone()} class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"/>
        </div>
    </div>
    }
}
