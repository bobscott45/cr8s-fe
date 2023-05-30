use yew::prelude::*;

use crate::components::input::Input;
use crate::components::button::Button;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
          <form class="space-y-6" action="#" method="POST">
           <Input label="Email address" input_type="email" name="email" required=true autocomplete="email" prompt=""/>
           <Input label="Password" input_type="password" name="password" required=true autocomplete="current-password" prompt="Forgot password?"/>
           <Button label="Sign in" button_type="submit"/>
          </form>
    }
}