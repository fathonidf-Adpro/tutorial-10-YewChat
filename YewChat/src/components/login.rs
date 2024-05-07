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
    <div class="bg-green-950 flex w-screen">
        <div class="container mx-auto flex flex-col justify-center items-center">
            // Menambahkan elemen img di sini
            <img src="https://github.com/fathonidf-Adpro/tutorial-10-broadcast-chat/assets/105644250/d0b10c60-6e7d-4265-8675-8493dee5a08c" alt="Deskripsi Gambar" class="mb-4" />
            <form class="m-4 flex">
                <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username" />
                <Link<Route> to={Route::Chat}>
                    <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-green-500 text-white font-bold p-4 uppercase border-green-500 border-t border-b border-r">
                        {"Go Chatting!"}
                    </button>
                </Link<Route>>
            </form>
        </div>
    </div>
}

}