use yew::prelude::*;

use crate::components::login_form::LoginForm;
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
              <LoginForm />
              <p class="mt-10 text-center text-sm text-gray-500">{"Not a member? "}<Link href="#" label="Start a 14 day free trial"/></p>
            </div>
      </div>
   }
}