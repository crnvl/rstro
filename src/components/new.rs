use yew::{function_component, Html, html};

#[function_component]
pub fn NewPost() -> Html {
    // get query params
    let window = web_sys::window().unwrap();
    let search = window.location().search().unwrap();
    let mut params = url::form_urlencoded::parse(search.trim_start_matches('?').as_bytes());
    let id = params.find(|(key, _)| key == "id").map(|(_, value)| value.to_string());

    let ref_id = match id {
        Some(id) => {
            html! {
                <input type="text" id="ref_id" name="ref_id" value={ id } />
            }
        }
        None => {
            html! {
                <input type="text" id="ref_id" name="ref_id" />
            }
        }
    };

    html! {
        <div id="container">
            <h1>{ "New Post" }</h1>
            <form action="https://ping.qwq.sh/new" method="POST">
                <label for="board">{ "Board: " }</label>
                <input type="text" id="board" name="board" /><br/>
                <label for="username">{ "Username: " }</label>
                <input type="text" id="username" name="username" /><br/>
                <label for="content">{ "Content: " }</label>
                <textarea id="content" name="content" cols="50"></textarea><br/>
                <label for="ref_id">{ "Reply to: " }</label>
                { ref_id }<br/>
                <input type="submit" value="Submit" />
            </form>
        </div>
    }
}