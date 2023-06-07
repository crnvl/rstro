use yew::prelude::*;

use crate::components::board::Board;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Board board="art" />
        </main>
    }
}
