use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{board::Board, home::Home};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/b/:board")]
    Board { board: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Board { board } => html! { <Board board={ board } /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
    <   /BrowserRouter>
    }
}
