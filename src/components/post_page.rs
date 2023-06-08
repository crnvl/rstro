use yew::{function_component, Html, html, Properties, use_effect_with_deps, use_state};

use crate::{utils::models::Post, components::post::PostEmbed};

#[derive(Properties, PartialEq)]
pub struct PostPageProps {
    pub id: String,
}

#[function_component]
pub fn PostPage(props: &PostPageProps) -> Html {
    let post = use_state(|| None);
    let post_clone = post.clone();

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