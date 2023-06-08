use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement};
use yew::{function_component, Html, Callback, html::onsubmit::Event, html};

use crate::utils::models::UserPost;

#[function_component]
pub fn NewPost() -> Html {
    let window = web_sys::window().unwrap();
    let search = window.location().search().unwrap();
    let mut params = url::form_urlencoded::parse(search.trim_start_matches('?').as_bytes());
    let id = params
        .find(|(key, _)| key == "id")
        .map(|(_, value)| value.to_string());

    let ref_id = match id {
        Some(id) => {
            html! {
                <input type="text" id="ref_id" class="ref_id" name="ref_id" value={ id } />
            }
        }
        None => {
            html! {
                <input type="text" id="ref_id" class="ref_id" name="ref_id" />
            }
        }
    };

    // callback fun stuff because this isn't javascript and i regret so many of my decisions
    let onchange = Callback::from(|event: Event| {
        event.prevent_default();
        let form = event.target().unwrap();
        let form_element: HtmlElement = form.unchecked_into();
        let board = form_element.get_elements_by_class_name("board").item(0).unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
        let username = form_element.get_elements_by_class_name("username").item(0).unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
        let content = form_element.get_elements_by_class_name("content").item(0).unwrap().dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().value();
        let ref_id = form_element.get_elements_by_class_name("ref_id").item(0).unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
        let thumb_url = form_element.get_elements_by_class_name("thumb_url").item(0).unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();

        wasm_bindgen_futures::spawn_local(async move {
            let endpoint = format!("https://ping.qwq.sh/posts/{}", board);
            let user_post = UserPost {
                thumb_url: if thumb_url.is_empty() { None } else { Some(thumb_url) },
                content,
                username: if username.is_empty() { None } else { Some(username) },
                ref_id: if ref_id.is_empty() { None } else { Some(ref_id) },
            };
            let request = Request::post(&endpoint)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&user_post).unwrap()).send();
            let response = request.await.unwrap();

            if response.status() == 200 {
                let window = web_sys::window().unwrap();
                window.location().set_href(&format!("/b/{}", board)).unwrap();
            } else {
                console::log_1(&"Error submitting post".into());
            }
        });
    });

    html! {
        <div id="container">
            <h1>{ "New Post" }</h1>
            <form onsubmit={onchange}>
                <label for="board">{ "Board: " }</label>
                <input class="board" type="text" id="board" name="board" required={true} /><br/>
                <label for="username">{ "Username: " }</label>
                <input class="username" type="text" id="username" name="username" value="anonymous" /><br/>
                <label for="content">{ "Content: " }</label>
                <textarea class="content" id="content" name="content" cols="50" required={true}></textarea><br/>
                <label for="thumb_url">{ "Image URL: " }</label>
                <input class="thumb_url" type="text" id="thumb_url" name="thumb_url" /><br/>
                <label for="ref_id">{ "Reply to: " }</label>
                { ref_id }<br/>
                <button type="submit">{ "Submit" }</button>
            </form>
        </div>
    }
}
