use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{board::Board, home::Home, post_page::PostPage};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/b/:board")]
    Board { board: String },
    #[at("/p/:id")]
    Post { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Board { board } => html! { <Board board={ board } /> },
        Route::Post { id } => html! { <PostPage id={ id } /> },
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
