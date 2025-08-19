use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod pages;

use components::navbar::Navbar;
use pages::{home::Home, blog::Blog, simulations::Simulations, resume::Resume};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/simulations")]
    Simulations,
    #[at("/resume")]
    Resume,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <Blog /> },
        Route::Simulations => html! { <Simulations /> },
        Route::Resume => html! { <Resume /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <Navbar />
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
