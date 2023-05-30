use yew::prelude::*;

use crate::components::input::Input;
use crate::components::button::Button;
use crate::components::link::Link;
use crate::components::logo::Logo;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
      <div class="flex mcin-h-full flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
          <Logo src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="My Company"/>
          <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">{"Sign in to your account"}</h2>
        </div>
        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
          <form class="space-y-6" action="#" method="POST">
           <Input label="Email address" input_type="email" name="email" required=true autocomplete="email" prompt=""/>
           <Input label="Password" input_type="password" name="password" required=true autocomplete="current-password" prompt="Forgot password?"/>
           <Button label="Sign in" button_type="submit"/>
          </form>

          <p class="mt-10 text-center text-sm text-gray-500">
            {"Not a member? "}
            <Link href="#" label="Start a 14 day free trial"/>
          </p>
        </div>
      </div>
    }
}