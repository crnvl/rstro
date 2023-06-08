use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

use crate::{components::post::PostEmbed, utils::models::Post};

#[derive(Properties, PartialEq)]
pub struct PostPageProps {
    pub id: String,
}

#[function_component]
pub fn PostPage(props: &PostPageProps) -> Html {
    let post = use_state(|| None);
    let post_clone = post.clone();

    let comments = use_state(|| None);
    let comments_clone = comments.clone();

    let id = props.id.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let endpoint = format!("https://ping.qwq.sh/post/{}", id);
                let fetched_post = reqwasm::http::Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json::<Post>()
                    .await
                    .unwrap();

                post.set(Some(fetched_post));

                let endpoint = format!("https://ping.qwq.sh/post/{}/comments", id);
                let fetched_comments = reqwasm::http::Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Post>>()
                    .await
                    .unwrap();

                comments.set(Some(fetched_comments));
            });
        },
        (),
    );

    html! {
        <>
            <h1>{ format!("/p/{}", props.id) }</h1>
            {
                match post_clone.as_ref() {
                    Some(p) => {
                        html! {
                            <>
                                <hr/>
                                <PostEmbed id={ p.id.clone() }
                                board={ p.board.clone() }
                                thumb_url={ p.thumb_url.clone() }
                                content={ p.content.clone() }
                                username={ p.username.clone() }
                                ref_id={ p.ref_id.clone() }
                                time={ p.time.clone() } />
                                {
                                    match comments_clone.as_ref() {
                                        Some(c) => {
                                            html! {
                                                <>
                                                    <hr/>
                                                    <h2>{ "Comments" }</h2>
                                                    {
                                                        for c.iter().map(|c| {
                                                            html! {
                                                                <>
                                                                    <PostEmbed id={ c.id.clone() }
                                                                    board={ c.board.clone() }
                                                                    thumb_url={ c.thumb_url.clone() }
                                                                    content={ c.content.clone() }
                                                                    username={ c.username.clone() }
                                                                    ref_id={ c.ref_id.clone() }
                                                                    time={ c.time.clone() } />
                                                                </>
                                                            }
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
