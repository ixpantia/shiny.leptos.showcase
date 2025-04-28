import "./shiny_leptos_dynamic_items_input";
import "./shiny_leptos_checkbox_group_buttons";
import "./shiny_leptos_text_input";
import "./shiny_leptos_counter_input";

if (Shiny.bindAll !== undefined) {
  console.log("shiny.leptos.showcase WASM loaded,  binding inputs and outputs");
  Shiny.bindAll(document.body)
} else {
  console.log("Shiny bindAll is  not yet defined. Shiny will bind by itself");
}
