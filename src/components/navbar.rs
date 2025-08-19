use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-container">
                <Link<Route> to={Route::Home} classes="nav-brand">
                    {"Gowtham Sundar"}
                </Link<Route>>
                <ul class="nav-menu">
                    <li class="nav-item">
                        <Link<Route> to={Route::Home} classes="nav-link">
                            {"Home"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Blog} classes="nav-link">
                            {"Blog"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Simulations} classes="nav-link">
                            {"Simulations"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Resume} classes="nav-link">
                            {"Resume"}
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
