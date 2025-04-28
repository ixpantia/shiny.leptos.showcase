import { RatePolicyModes } from "rstudio-shiny/srcts/types/src/inputPolicies/inputRateDecorator";
import {
  attach_shiny_leptos_dynamic_items_input,
  update_shiny_leptos_dynamic_items_input,
  get_shiny_leptos_dynamic_items_input_value,
  ShinyLeptosDynamicItemsInputState,
  subscribe_shiny_leptos_dynamic_items_input
} from "shiny_leptos_showcase-wasm";

// Define the interface for the state object managed by WASM
// Note: This is an opaque type reference; we don't interact with its fields directly in TS.
// declare class ShinyLeptosDynamicItemsInputState {} // Or keep existing if already there

// Extend the HTMLElement interface to include our state property
interface HTMLElement {
    state: ShinyLeptosDynamicItemsInputState | undefined;
}

class ShinyLeptosDynamicItemsInputBinding extends window.Shiny.InputBinding {

  find(scope: HTMLElement): JQuery<HTMLElement> {
    // Ensure this class matches the one in the R function
    return $(scope).find(".shiny_leptos_showcase-shiny_leptos_dynamic_items_input-container");
  }

  initialize(el: HTMLElement): void {
    const id = el.id;
    if (!id) return;

    const initialValueStr = el.getAttribute("data-initial-value");
    let initialValue: unknown[] | null = null;
    try {
        // Parse the JSON string from the R side
        initialValue = initialValueStr ? JSON.parse(initialValueStr) : [];
        if (!Array.isArray(initialValue)) {
            console.error(`Initial value for ${id} is not an array:`, initialValue);
            initialValue = [];
        }
    } catch (e) {
        console.error(`Error parsing initial value for ${id}:`, e);
        initialValue = [];
    }

    try {
        // Attach the Leptos component, passing the parsed array (as JsValue via wasm-bindgen)
        el.state = attach_shiny_leptos_dynamic_items_input(el, initialValue);
    } catch (e) {
         console.error(`Error attaching shiny_leptos_dynamic_items_input for ${id}:`, e);
         el.state = undefined; // Ensure state is undefined on error
    }
  }

  subscribe(el: HTMLElement, callback: (value: boolean) => void): void {
    if (!el.state) {
        // Attempt to initialize if state doesn't exist (e.g., error during initial attach)
        // This might happen if the WASM load was delayed after the initial Shiny scan.
        this.initialize(el);
        // If still no state after re-initializing, we can't subscribe.
        if (!el.state) {
            console.error(`Cannot subscribe to ${el.id}: state is missing.`);
            return;
        }
    }
    // Subscribe to changes in the Rust state
    subscribe_shiny_leptos_dynamic_items_input(el.state, callback);
  }

  getValue(el: HTMLElement): any {
    if (!el.state) {
        console.warn(`Getting value for ${el.id} before initialization or after error.`);
        // Return a default value consistent with the expected type (an array)
        // Try to read initial value again as a fallback, otherwise empty array
        const initialValueStr = el.getAttribute("data-initial-value");
        try {
             const initialValue = initialValueStr ? JSON.parse(initialValueStr) : [];
             return Array.isArray(initialValue) ? initialValue : [];
        } catch {
            return [];
        }
    }
    try {
        // Get the current value from the Rust state (returns JsValue representing array)
        return get_shiny_leptos_dynamic_items_input_value(el.state);
    } catch (e) {
        console.error(`Error getting value for ${id}:`, e);
        return []; // Return empty array on error
    }
  }

  setValue(el: HTMLElement, value: any): void {
     if (!el.state) {
         console.warn(`Setting value for ${el.id} before initialization or after error.`);
         // Attempt to initialize state before setting value
         this.initialize(el);
         if (!el.state) {
              console.error(`Cannot set value for ${el.id}: state is missing.`);
              return;
         }
     }
     // Ensure the value is an array before sending it to WASM
     if (!Array.isArray(value)) {
        console.error(`Cannot set value for ${el.id}: received non-array value:`, value);
        value = []; // Or handle the error more gracefully depending on requirements
     }
     try {
        // Update the Rust state with the new value (passed as JsValue)
        update_shiny_leptos_dynamic_items_input(el.state, value);
     } catch(e) {
         console.error(`Error setting value for ${el.id}:`, e);
     }
  }

  receiveMessage(el: HTMLElement, data: any): void {
    if (data.hasOwnProperty('value')) {
        this.setValue(el, data.value);
    }
    // Potentially handle other message types like label updates, options changes etc.
    // if (data.hasOwnProperty('label')) { ... }
  }

  getRatePolicy(el: HTMLElement): { policy: RatePolicyModes; delay: number; } | null {
    // This input updates reactively, no specific rate policy needed from Shiny's side
    return null;
  }

  getType(): string | null {
     // Let Shiny infer the type based on the value (which will be an array)
     // Returning null or undefined is often sufficient here.
    return null;
  }
}

// Register the binding with Shiny
window.Shiny.inputBindings.register(new ShinyLeptosDynamicItemsInputBinding(), "ShinyLeptosDynamicItemsInputBinding");

// Export the state type if needed elsewhere (usually not necessary for input bindings)
export type { ShinyLeptosDynamicItemsInputState };
