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
        // Ganti background jadi rose-100 dan tambah h-screen biar full layar
        <div class="bg-rose-100 flex h-screen w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">

                // Nambahin teks Judul di halaman depan
                <h1 class="text-4xl font-bold text-rose-800 mb-6 drop-shadow-md">{"✨ My Messenger ✨"}</h1>

                <form class="m-4 flex shadow-xl rounded-lg">
                    // Ganti warna border dan placeholder
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-rose-900 border-rose-300 bg-white focus:outline-none focus:ring-2 focus:ring-rose-400" placeholder="Who are you? (e.g. Tsaniya)" />
                    <Link<Route> to={Route::Chat}>
                        // Ganti warna tombol jadi rose-500 dan ganti teksnya
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-rose-500 hover:bg-rose-600 text-white font-bold p-4 uppercase border-rose-500 border-t border-b border-r transition-colors">
                            {"Enter!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
