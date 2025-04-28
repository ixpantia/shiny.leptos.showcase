import { RatePolicyModes } from "rstudio-shiny/srcts/types/src/inputPolicies/inputRateDecorator";
import {
  attach_shiny_leptos_counter_input,
  update_shiny_leptos_counter_input,
  get_shiny_leptos_counter_input_value,
  ShinyLeptosCounterInputState,
  subscribe_shiny_leptos_counter_input
} from "shiny_leptos_showcase-wasm";

interface HTMLElement {
    state: ShinyLeptosCounterInputState | undefined;
}

// Button:
class ShinyLeptosCounterInputBinding extends window.Shiny.InputBinding {

  find(scope: HTMLElement): JQuery<HTMLElement> {
    return $(scope).find(".shiny_leptos_showcase-shiny_leptos_counter_input-container");
  }

  initialize(el: HTMLElement): void {
    const id = el.id;
    if (!id) return;

    const initialValue = el.getAttribute("data-initial-value");
    const parsedValue = initialValue ? parseFloat(initialValue) : null;

    el.state = attach_shiny_leptos_counter_input(el, parsedValue);
  }

  subscribe(el: HTMLElement, callback: (value: boolean) => void): void {
    if (!el.state) this.initialize(el);
    if (el.state) {
      subscribe_shiny_leptos_counter_input(el.state, callback);
    }
  }

  getValue(el: HTMLElement): any {
    if (!el.state) this.initialize(el);
    return get_shiny_leptos_counter_input_value(el.state);
  }

  setValue(el: HTMLElement, value: any): void {
    if (!el.state) this.initialize(el);
    update_shiny_leptos_counter_input(el.state, value);
  }

  receiveMessage(el: HTMLElement, data: any): void {
    if (!el.state) this.initialize(el);
    this.setValue(el, data.value);
  }

  getRatePolicy(el: HTMLElement): { policy: RatePolicyModes; delay: number; } | null {
    return null;
  }

  getType() {
    return "";
  }
}

window.Shiny.inputBindings.register(new ShinyLeptosCounterInputBinding(), "ShinyLeptosCounterInputBinding");
