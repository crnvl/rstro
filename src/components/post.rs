use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}

#[function_component]
pub fn PostEmbed(post: &PostProps) -> Html {
    html! {
        <div id="container">
            <p><a href={format!("/p/{}", post.ref_id)}>{ 
                if post.ref_id != "0" {
                    format!(">>#{}", post.ref_id)
                } else {
                    "".to_string()
                }
             }</a></p>
            <p><b>{ format!("by {} | #{} | {}", post.username.clone(), post.id, post.time.clone()) }</b></p> 
            <div id="post-wrap">
                {
                    if post.thumb_url != "" {
                        html! {
                            <div id="post-left">
                                <img id="post-img" src={ post.thumb_url.clone() } />
                                <h6><a href={ post.thumb_url.clone() } target="_blank" rel="noopener noreferrer">{"View full image"}</a></h6>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div id="post-right">
                    <p>{ post.content.clone() }</p>
                </div>
            </div>
        </div>
    }
}
