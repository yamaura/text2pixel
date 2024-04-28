use leptos::*;
use text2pixel::*;

fn text2datapng(text: &str) -> String {
    use base64::prelude::*;

    if text.is_empty() {
        return String::new();
    }
    let data = text2png(text);
    let data = BASE64_STANDARD.encode(&data);
    format!("data:image/png;base64,{}", data)
}

#[component]
fn App() -> impl IntoView {
    let (text, set_text) = create_signal(String::new());
    view! {
        <textarea on:keyup=move |e| { set_text(event_target_value(&e)) }></textarea>
            <div>
        <img src=move || text2datapng(&text.get())/>
        </div>
    }
}

//#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}