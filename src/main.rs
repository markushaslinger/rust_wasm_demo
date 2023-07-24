use leptos::*;
use chrono::Local;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (items, set_items) = create_signal(cx, vec![]);
    let (count, set_count) = create_signal(cx, 0);

    let on_click = move |_| {
        set_count.update(|count| {
            *count += 1;
            let timestamp = Local::now().to_string();
            let mut new_items = items.get();
            new_items.push(format!("Click count: {count:02}, Timestamp: {timestamp}"));
            set_items.set(new_items);
        });
    };

    view! { cx,
        <h1>"Welcome to Rust-WASM@HTL-Leonding"</h1>
        <span>Using the Leptos framework</span>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div class="list-container">
            <ul>
                {move || items.get().into_iter().rev()
                    .map(|item| view!{ cx, <li>{item}</li>})
                    .collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}