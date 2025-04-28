use leptos::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{js_sys, HtmlElement};

// This struct will hold the state of the component
#[derive(Clone)]
#[wasm_bindgen]
pub struct ShinyLeptosColorPickerInputState {
    value: RwSignal<String>, // Store color as a hex string
}

#[component]
fn ShinyLeptosColorPickerInput(state: ShinyLeptosColorPickerInputState) -> impl IntoView {
    // Define the rendering logic using an HTML input type="color"
    view! {
        <input
            type="color"
            class="form-control form-control-color" // Use Bootstrap's color input class
            // `bind:value` provides two-way binding for input elements
            bind:value=state.value
        />
    }
}

// Attach the component to a DOM element and initialize its state
#[wasm_bindgen]
pub fn attach_shiny_leptos_color_picker_input(
    element: HtmlElement,
    initial_value: Option<String>,
) -> Result<ShinyLeptosColorPickerInputState, JsValue> {
    let initial_value = initial_value.unwrap_or_else(|| "#000000".to_string());
    // Validate initial value format slightly
    if !initial_value.starts_with('#') || initial_value.len() != 7 {
        web_sys::console::warn_1(&"Initial color value format incorrect, using #000000".into());
        // Use default instead of potentially invalid input
        let value = RwSignal::new("#000000".to_string());
        let state = ShinyLeptosColorPickerInputState { value };
        let component_state = state.clone();
        leptos::mount::mount_to(element, move || {
            view! { <ShinyLeptosColorPickerInput state=component_state/> }
        })
        .forget();
        return Ok(state);
    }

    let value = RwSignal::new(initial_value);
    let state = ShinyLeptosColorPickerInputState { value };
    let component_state = state.clone();

    leptos::mount::mount_to(element, move || {
        view! { <ShinyLeptosColorPickerInput state=component_state/> }
    })
    .forget();

    Ok(state)
}

// Update the component's state
#[wasm_bindgen]
pub fn update_shiny_leptos_color_picker_input(
    state: &ShinyLeptosColorPickerInputState,
    value: String,
) -> Result<(), JsValue> {
    // Basic validation
    if !value.starts_with('#') || value.len() != 7 {
        let error_msg = format!("Invalid color format: '{}'. Expected '#RRGGBB'.", value);
        web_sys::console::error_1(&error_msg.into());
        // Optionally return an error or just log and don't update
        // return Err(JsValue::from_str(&error_msg));
        return Ok(()); // Don't update if invalid
    }
    state.value.set(value);
    Ok(())
}

// Get the current value
#[wasm_bindgen]
pub fn get_shiny_leptos_color_picker_input_value(
    state: &ShinyLeptosColorPickerInputState,
) -> String {
    state.value.get_untracked()
}

// Subscribe to changes
#[wasm_bindgen]
pub fn subscribe_shiny_leptos_color_picker_input(
    state: &ShinyLeptosColorPickerInputState,
    callback: js_sys::Function,
) {
    let state = state.clone();
    Effect::watch(
        move || state.value.get(),
        move |_, _, _| match callback.call0(&JsValue::FALSE) {
            Ok(_) => {}
            Err(err) => {
                web_sys::console::error_1(
                    &format!(
                        "Error calling color picker callback: {}",
                        err.as_string().unwrap_or_default()
                    )
                    .into(),
                );
            }
        },
        false, // Do not run immediately
    );
}
