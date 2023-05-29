use yew::prelude::*;

mod pages;
mod components;

use pages::login::Login;

#[function_component(App)]
fn app() -> Html {
    html! {
     <Login />
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
