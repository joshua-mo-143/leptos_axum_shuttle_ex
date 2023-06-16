use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div class="w-screen h-screen">
        <div class="my-0 mx-auto py-20 max-w-3xl text-center flex flex-col justify-center items-center">
            <h2 class="p-6 text-2xl text-center font-bold">"Welcome to Leptos with Tailwind & Axum"</h2>
            <div class="grid grid-cols-2 grid-rows-2 gap-2 min-h-96 h-auto w-4/5">
                <div class="text-center  rounded-md shadow-md p-4 flex flex-col gap-2 items-center bg-white/10">
                    <h2 class="text-center text-lg font-bold">"Built with Leptos"</h2>
                <p class="text-md">"Leptos is a Rust webdev framework that leverages fine-grained reactivity to build declarative user interfaces."</p>
                    <LinkButton link="https://www.google.com"/>
                    </div>
                <div class="text-center  rounded-md shadow-md p-4 flex flex-col gap-2 items-center bg-white/10">
                    <h2 class="text-center text-lg font-bold">"Powered by Axum"</h2>
                <p class="text-md">"Axum is a web backend framework with ergonomic, uncomplicated syntax that makes building services easy."</p>
                    <LinkButton link="https://www.google.com"/>
                    </div>
                <div class="text-center  rounded-md shadow-md p-4 flex flex-col gap-2 items-center bg-white/10">
                    <h2 class="text-center text-lg font-bold">"Developed in Rust"</h2>
                <p class="text-md">"Rust is a programming language built to empower developers to build safe and reliable software, without hassle."</p>
                    <LinkButton link="https://www.google.com"/>
                    </div>
                <div class="text-center  rounded-md shadow-md p-4 flex flex-col gap-2 items-center bg-white/10">
                    <h2 class="text-center text-lg font-bold">"Deployed on Shuttle"</h2>
                <p class="text-md">"Shuttle is a Rust-native cloud dev platform aimed at catering to a uniquely excellent DX and first-class Rust support."</p>
                    <LinkButton link="https://www.google.com"/>
                    </div>
                </div>
                </div>
        </div>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! {cx,
    <nav class="w-screen h-10 bg-green-300">
        <ul class="flex flex-row items-center px-4">
        <li>"Hello world!"</li>
        </ul>
    </nav>}
}

#[component]
fn LinkButton(cx: Scope, link: &'static str) -> impl IntoView {
    view! {cx,
    <a href=link target="_blank" class="bg-sky-500 hover:bg-sky-400 transition-all text-black text-md w-max px-5 rounded-md"><p>"Find out more"</p></a>}
}
