import { RatePolicyModes } from "rstudio-shiny/srcts/types/src/inputPolicies/inputRateDecorator";
import {
  attach_shiny_leptos_color_picker_input,
  update_shiny_leptos_color_picker_input,
  get_shiny_leptos_color_picker_input_value,
  ShinyLeptosColorPickerInputState,
  subscribe_shiny_leptos_color_picker_input
} from "shiny_leptos_showcase-wasm";

// Extend HTMLElement to include our state property
interface HTMLElement {
    state: ShinyLeptosColorPickerInputState | undefined;
}

class ShinyLeptosColorPickerInputBinding extends window.Shiny.InputBinding {

  find(scope: HTMLElement): JQuery<HTMLElement> {
    return $(scope).find(".shiny_leptos_showcase-shiny_leptos_color_picker_input-container");
  }

  initialize(el: HTMLElement): void {
    const id = el.id;
    if (!id) return;

    const initialValue = el.getAttribute("data-initial-value"); // Can be null
    // Pass initialValue (string or null) directly to attach function
    try {
      el.state = attach_shiny_leptos_color_picker_input(el, initialValue);
    } catch (e) {
       console.error(`Error attaching shiny_leptos_color_picker_input for ${id}:`, e);
       el.state = undefined;
    }
  }

  subscribe(el: HTMLElement, callback: (value: boolean) => void): void {
    if (!el.state) {
      this.initialize(el); // Attempt re-initialization
      if (!el.state) {
        console.error(`Cannot subscribe to ${el.id}: state is missing.`);
        return;
      }
    }
    subscribe_shiny_leptos_color_picker_input(el.state, callback);
  }

  getValue(el: HTMLElement): string | null { // Return type is string or null
    if (!el.state) {
      console.warn(`Getting value for ${el.id} before initialization or after error.`);
      // Return initial value as fallback, or default if not available
       return el.getAttribute("data-initial-value") ?? "#000000";
    }
    try {
      return get_shiny_leptos_color_picker_input_value(el.state);
    } catch (e) {
      console.error(`Error getting value for ${el.id}:`, e);
      return el.getAttribute("data-initial-value") ?? "#000000"; // Fallback
    }
  }

  setValue(el: HTMLElement, value: string | null): void { // Accept string or null
     if (!el.state) {
         console.warn(`Setting value for ${el.id} before initialization or after error.`);
         this.initialize(el);
         if (!el.state) {
              console.error(`Cannot set value for ${el.id}: state is missing.`);
              return;
         }
     }
     // Ensure value is a string; handle null by using a default or ignoring
     const colorString = (typeof value === 'string') ? value : "#000000"; // Default if null/undefined

     // Basic validation (optional, Rust side does it too)
      if (!/^#[0-9a-fA-F]{6}$/.test(colorString)) {
          console.warn(`Invalid color format received in setValue for ${el.id}: ${value}. Using default.`);
          // You might want to update with a default or skip the update
          // For now, let's try updating with the potentially invalid value and let Rust handle/warn
          // Or force a default: colorString = "#000000";
      }

     try {
        update_shiny_leptos_color_picker_input(el.state, colorString);
     } catch(e) {
         console.error(`Error setting value for ${el.id}:`, e);
     }
  }

  receiveMessage(el: HTMLElement, data: any): void {
    if (data.hasOwnProperty('value')) {
        this.setValue(el, data.value);
    }
    // Handle label updates if you added label support back
    // if (data.hasOwnProperty('label')) { ... }
  }

  getRatePolicy(el: HTMLElement): { policy: RatePolicyModes; delay: number; } | null {
    // Input type="color" typically fires 'input' or 'change' events.
    // No specific rate policy usually needed from Shiny's side unless debouncing is desired.
    return null;
  }

  getType(): string {
     // Tell Shiny this input's value is text
    return "";
  }
}

// Register the binding with Shiny
window.Shiny.inputBindings.register(new ShinyLeptosColorPickerInputBinding(), "ShinyLeptosColorPickerInputBinding");

// Export the state type if needed elsewhere
export type { ShinyLeptosColorPickerInputState };
