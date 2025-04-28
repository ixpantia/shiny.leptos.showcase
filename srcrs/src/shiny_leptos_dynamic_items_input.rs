use leptos::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{js_sys, HtmlElement};

// This struct will hold the state of the component
#[derive(Clone)]
#[wasm_bindgen]
pub struct ShinyLeptosDynamicItemsInputState {
    items: RwSignal<Vec<String>>, // Holds the list of items
}

#[component]
fn ShinyLeptosDynamicItemsInput(state: ShinyLeptosDynamicItemsInputState) -> impl IntoView {
    let new_item_text = RwSignal::new("".to_string());

    fn add_item(items: RwSignal<Vec<String>>, new_item_text: RwSignal<String>) {
        let new_item = new_item_text.get();
        if !new_item.trim().is_empty() {
            items.update(|items| items.push(new_item));
            new_item_text.update(|text| text.clear()); // Clear the input field after adding
        }
    }

    let remove_item = move |index_to_remove: usize| {
        state.items.update(|items| {
            if index_to_remove < items.len() {
                items.remove(index_to_remove);
            }
        });
    };

    view! {
        <div class="shiny-leptos-dynamic-items-input">
            <div class="input-group mb-3">
                <input
                    type="text"
                    class="form-control"
                    placeholder="Add new item"
                    bind:value=new_item_text
                    on:keydown=move |ev| { // Allow adding with Enter key
                        if ev.key() == "Enter" {
                             ev.prevent_default(); // Prevent form submission if inside a form
                             add_item(state.items, new_item_text);
                        }
                    }
                />
                <button class="btn btn-outline-secondary" type="button" on:click=move |_| add_item(state.items, new_item_text)>"Add"</button>
            </div>
            <ul class="list-group">
                <For
                    each=move || state.items.get().into_iter().enumerate()
                    key=| (index, item) | (*index, item.clone()) // Use index and item for key
                    children=move |(index, item)| {
                        view! {
                            <li class="list-group-item d-flex justify-content-between align-items-center">
                                {item}
                                <button
                                    class="btn btn-danger btn-sm"
                                    on:click=move |_| remove_item(index)
                                >
                                 "×" // Unicode multiplication sign for 'x'
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[wasm_bindgen]
pub fn attach_shiny_leptos_dynamic_items_input(
    element: HtmlElement,
    initial_value: JsValue, // Accept JsValue directly
) -> Result<ShinyLeptosDynamicItemsInputState, JsValue> {
    let initial_items: Vec<String> =
        serde_wasm_bindgen::from_value(initial_value).unwrap_or_else(|_| Vec::new());

    let items = RwSignal::new(initial_items);

    let state = ShinyLeptosDynamicItemsInputState { items };
    let component_state = state.clone();

    leptos::mount::mount_to(element, move || {
        view! {
            <ShinyLeptosDynamicItemsInput state=component_state />
        }
    })
    .forget();

    Ok(state)
}

#[wasm_bindgen]
pub fn update_shiny_leptos_dynamic_items_input(
    state: &ShinyLeptosDynamicItemsInputState,
    value: JsValue, // Accept JsValue directly
) -> Result<(), JsValue> {
    let new_items: Vec<String> =
        serde_wasm_bindgen::from_value(value).unwrap_or_else(|_| Vec::new());
    state.items.set(new_items);
    Ok(())
}

#[wasm_bindgen]
pub fn get_shiny_leptos_dynamic_items_input_value(
    state: &ShinyLeptosDynamicItemsInputState,
) -> Result<JsValue, JsValue> {
    // Convert Vec<String> to JsValue (will be a JS array)
    Ok(serde_wasm_bindgen::to_value(&state.items.get_untracked())?)
}

#[wasm_bindgen]
pub fn subscribe_shiny_leptos_dynamic_items_input(
    state: &ShinyLeptosDynamicItemsInputState,
    callback: js_sys::Function,
) {
    let state = state.clone();
    Effect::watch(
        move || state.items.get(),
        move |_, _, _| match callback.call0(&JsValue::FALSE) {
            Ok(_) => {}
            Err(err) => {
                web_sys::console::error_1(
                    &format!(
                        "Error calling callback: {}",
                        err.as_string().unwrap_or_default()
                    )
                    .into(),
                );
            }
        },
        false, // Do not run immediately
    );
}
