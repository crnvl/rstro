use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

use crate::{components::post::PostEmbed, utils::models::Post};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: String,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    let posts = use_state(|| None);
    let posts_clone = posts.clone();

    let board = props.board.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let endpoint = format!("https://ping.qwq.sh/posts/{}", board);
                let fetched_posts = Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Post>>()
                    .await
                    .unwrap();

                posts.set(Some(fetched_posts));
            });
        },
        (),
    );

    match posts_clone.as_ref() {
        Some(p) => {
            html! {
                <>
                    <h1>{ format!("/b/{}", props.board) }</h1>
                    { for p.iter().map(|post|
                        html! {
                            <>
                                <hr/>
                                <PostEmbed id={ post.id.clone() }
                                board={ post.board.clone() }
                                thumb_url={ post.thumb_url.clone() }
                                content={ post.content.clone() }
                                username={ post.username.clone() }
                                ref_id={ post.ref_id.clone() }
                                time={ post.time.clone() } />
                                <h6><a href={ format!("/p/{}", post.id) }>{ "> View thread" }</a> { " | " } <a href={ format!("/new?id={}", post.id) }>{ "Reply" }</a></h6>
                            </>
                        })
                     }
                </>
            }
        }
        None => {
            html! {
                <>
                    <p>{ "Loading..." }</p>
                </>
            }
        }
    }
}
