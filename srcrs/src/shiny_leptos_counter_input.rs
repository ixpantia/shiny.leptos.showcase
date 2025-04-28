use leptos::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{ HtmlElement, js_sys };

// This struct will hold the state of the component
#[derive(Clone)]
#[wasm_bindgen]
pub struct ShinyLeptosCounterInputState {
    value: RwSignal<f64>,
}

#[component]
fn ShinyLeptosCounterInput(state: ShinyLeptosCounterInputState) -> impl IntoView {
    // Define the rendering logic of your input component
    view! {
        <button class="btn btn-primary" on:click=move |_| *state.value.write() += 1.0 >
            "Count: " {state.value}
        </button>
    }
}

// This function will be called from JavaScript to attach the component to a DOM element
// and to initialize its state
#[wasm_bindgen]
pub fn attach_shiny_leptos_counter_input(element: HtmlElement, initial_value: Option<f64>) -> ShinyLeptosCounterInputState {
    let initial_value = initial_value.unwrap_or(0.0);
    let value = RwSignal::new(initial_value);
    let state = ShinyLeptosCounterInputState { value };
    let component_state = state.clone();
    leptos::mount::mount_to(element, move || {
        view! {
            <ShinyLeptosCounterInput state=component_state></ShinyLeptosCounterInput>
        }
    })
    .forget();
    state
}

// This function will be called from JavaScript to update the state of the component
// without re-initializing it
#[wasm_bindgen]
pub fn update_shiny_leptos_counter_input(state: &ShinyLeptosCounterInputState, value: f64) {
    state.value.set(value);
}

// This function will be called from JavaScript to get the current value of the component
// at any time
#[wasm_bindgen]
pub fn get_shiny_leptos_counter_input_value(state: &ShinyLeptosCounterInputState) -> f64 {
    state.value.get_untracked()
}

// This function will be called from JavaScript to subscribe to changes in the component's state.
// The callback will be called whenever the state changes telling Shiny to
// send the new value to the server
#[wasm_bindgen]
pub fn subscribe_shiny_leptos_counter_input(state: &ShinyLeptosCounterInputState, callback: js_sys::Function) {
    let state = state.clone();
    Effect::watch(
        move || state.value.get(),
        // (value, last_value, _prev)
        // The value passed to the callback is FALSE since we do not
        // have a set rate policy
        move |_, _, _| match callback.call0(&JsValue::FALSE) {
            Ok(_) => {}
            Err(err) => {
                web_sys::console::error_1(&JsValue::from_str(&format!(
                    "Error calling callback: {}",
                    err.as_string().unwrap_or_default()
                )));
            }
        },
        // Do not run the effect immediately
        false,
    );
}
