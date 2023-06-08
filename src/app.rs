use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{board::Board, home::Home, post_page::PostPage, new::NewPost};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/b/:board")]
    Board { board: String },
    #[at("/p/:id")]
    Post { id: String },
    #[at("/new")]
    New,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Board { board } => html! { <Board board={ board } /> },
        Route::Post { id } => html! { <PostPage id={ id } /> },
        Route::New => html! { <NewPost /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <div id="header">
                <a href="/">{ "Home" }</a>
                <a href="/new">{ "New Post" }</a>
                <a href="https://github.com/angelsflyinhell/rstro">{ "rstro" }</a>
                <a href="https://github.com/angelsflyinhell/ping">{ "ping" }</a>
            </div>
            <div id="content"> 
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </>
    }
}
