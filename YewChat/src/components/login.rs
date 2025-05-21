use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-full flex flex-col items-center justify-center bg-gradient-to-tr from-blue-500 via-indigo-600 to-purple-700 font-mono text-white px-6 py-12">
            <div class="max-w-xl w-full bg-white bg-opacity-20 backdrop-blur-lg rounded-xl shadow-lg p-8 text-center">
                <h1 class="text-4xl font-extrabold mb-4">{"👋 Welcome!"}</h1>
                <p class="mb-8 text-lg opacity-90">
                    {"Jump into the future of chatting 🚀"}
                </p>
                <form
                    onsubmit={Callback::from(|e: FocusEvent| e.prevent_default())}
                    class="flex flex-col md:flex-row items-center gap-4 max-w-md mx-auto"
                >
                    <input
                        type="text"
                        placeholder="Enter your username..."
                        class="w-full p-4 text-gray-900 rounded-full outline-none text-base border-2 border-transparent focus:border-indigo-300 transition-all shadow-md"
                        {oninput}
                        value={(*username).clone()}
                        required=true
                        autocomplete="off"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            type="button"
                            onclick={onclick}
                            disabled={username.is_empty()}
                            class="w-full md:w-auto bg-indigo-600 disabled:bg-indigo-400 text-white px-8 py-4 font-bold rounded-full hover:bg-indigo-700 hover:shadow-lg transform hover:-translate-y-1 transition-all duration-200 text-base uppercase tracking-wide"
                        >
                            {"Join Chat"}
                        </button>
                    </Link<Route>>
                </form>
                <div class="mt-6 text-sm italic opacity-80">
                    {"Your creativity is your superpower — let's chat and create together!"}
                </div>
                <div class="mt-6 flex justify-center space-x-6 text-sm">
                    <a href="https://www.weforum.org/agenda/2020/11/ai-automation-creativity-workforce-skill-fute-of-work/" target="_blank" rel="noopener noreferrer" class="hover:underline hover:text-white transition-colors">
                        {"Why Creativity?"}
                    </a>
                    <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer" class="hover:underline hover:text-white transition-colors">
                        {"Powered by Yew"}
                    </a>
                </div>
            </div>
        </div>
    }
}