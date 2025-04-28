use leptos::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{js_sys, HtmlElement};

#[derive(serde::Deserialize, Clone)]
pub struct CheckboxGroupButtonsOptions {
    options: Vec<String>,
    labels: Vec<String>,
}

impl CheckboxGroupButtonsOptions {
    pub fn iter(self) -> impl Iterator<Item = (String, String)> {
        self.options.into_iter().zip(self.labels)
    }
}

// This struct will hold the state of the component
#[derive(Clone)]
#[wasm_bindgen]
pub struct ShinyLeptosCheckboxGroupButtonsState {
    selected: RwSignal<Option<String>>,
    options: RwSignal<CheckboxGroupButtonsOptions>,
}

#[component]
fn ShinyLeptosCheckboxGroupButtons(state: ShinyLeptosCheckboxGroupButtonsState) -> impl IntoView {
    // Define the rendering logic of your input component
    view! {
        <For
            each=move || state.options.get().iter()
            key=|value| value.clone()
            children=move |(option, label)| {
                let (option, _) = signal(option);
                view! {
                    <button
                        on:click=move |_| state.selected.set(Some(option.get()))
                        class:btn=true
                        class:btn-primary-outline=true
                        class=move || match state.selected.get() {
                            Some(s) if s == option.get() => "btn btn-primary",
                            _ => "btn btn-outline-primary",
                        }
                    >
                    {label}
                    </button>
                }
            }
        />
    }
}

// This function will be called from JavaScript to attach the component to a DOM element
// and to initialize its state
#[wasm_bindgen]
pub fn attach_shiny_leptos_checkbox_group_buttons(
    element: HtmlElement,
    initial_value: Option<String>,
    options: JsValue,
) -> Result<ShinyLeptosCheckboxGroupButtonsState, String> {
    let options =
        RwSignal::new(serde_wasm_bindgen::from_value(options).map_err(|e| e.to_string())?);
    let selected = RwSignal::new(initial_value);
    let state = ShinyLeptosCheckboxGroupButtonsState { selected, options };
    let component_state = state.clone();
    leptos::mount::mount_to(element, move || {
        view! {
            <ShinyLeptosCheckboxGroupButtons state=component_state></ShinyLeptosCheckboxGroupButtons>
        }
    })
    .forget();
    Ok(state)
}

// This function will be called from JavaScript to update the state of the component
// without re-initializing it
#[wasm_bindgen]
pub fn update_shiny_leptos_checkbox_group_buttons(
    state: &ShinyLeptosCheckboxGroupButtonsState,
    value: Option<String>,
) -> Result<(), String> {
    state.selected.set(value);
    Ok(())
}

// This function will be called from JavaScript to get the current value of the component
// at any time
#[wasm_bindgen]
pub fn get_shiny_leptos_checkbox_group_buttons_value(
    state: &ShinyLeptosCheckboxGroupButtonsState,
) -> Option<String> {
    state.selected.get_untracked()
}

// This function will be called from JavaScript to subscribe to changes in the component's state.
// The callback will be called whenever the state changes telling Shiny to
// send the new value to the server
#[wasm_bindgen]
pub fn subscribe_shiny_leptos_checkbox_group_buttons(
    state: &ShinyLeptosCheckboxGroupButtonsState,
    callback: js_sys::Function,
) {
    let state = state.clone();
    Effect::watch(
        move || state.selected.get(),
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
