use yew::{Html, function_component, html, use_state, use_effect_with_deps};

#[function_component]
pub fn Home() -> Html {
    let boards = use_state(|| None);
    let boards_clone = boards.clone();

    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_boards = reqwasm::http::Request::get("https://ping.qwq.sh/boards")
                .send()
                .await
                .unwrap()
                .json::<Vec<String>>()
                .await
                .unwrap();

            boards.set(Some(fetched_boards));
        });
    }, ());

    html! {
        <>
            <h1>{ "Welcome to ping!" }</h1>
            <p>{ "Ping is a barebones & anonymous social network as a web server built with Rust. You're currently using rstro as a client." }</p>
            <p>{ "Right now, this client cannot be used to create posts, but you can take a look at other people's posts on the boards listed below!" }</p>
            <h2>{ "Boards" }</h2>
            {
                match boards_clone.as_ref() {
                    Some(b) => {
                        html! {
                            <>
                                { for b.iter().map(|board|
                                    html! {
                                        <>
                                            <a href={ format!("/b/{}", board) }><b>{ format!("/b/{}", board) }</b></a><br />
                                        </>
                                    })
                                }
                            </>
                        }
                    },
                    None => {
                        html! {
                            <>
                                <p>{ "Loading..." }</p>
                            </>
                        }
                    }
                }
            }
        </>
    }
}