import { RatePolicyModes } from "rstudio-shiny/srcts/types/src/inputPolicies/inputRateDecorator";
import {
  attach_shiny_leptos_checkbox_group_buttons,
  update_shiny_leptos_checkbox_group_buttons,
  get_shiny_leptos_checkbox_group_buttons_value,
  ShinyLeptosCheckboxGroupButtonsState,
  subscribe_shiny_leptos_checkbox_group_buttons
} from "shiny_leptos_showcase-wasm";

interface HTMLElement {
    state: ShinyLeptosCheckboxGroupButtonsState | undefined;
}

// Button:
class ShinyLeptosCheckboxGroupButtonsBinding extends window.Shiny.InputBinding {

  find(scope: HTMLElement): JQuery<HTMLElement> {
    return $(scope).find(".shiny_leptos_showcase-shiny_leptos_checkbox_group_buttons-container");
  }

  initialize(el: HTMLElement): void {
    const id = el.id;
    if (!id) return;

    const options = JSON.parse(el.getAttribute("data-options") || "{}");
    const initialValue = el.getAttribute("data-initial-value");

    el.state = attach_shiny_leptos_checkbox_group_buttons(el, initialValue, options);
  }

  subscribe(el: HTMLElement, callback: (value: boolean) => void): void {
    if (!el.state) this.initialize(el);
    if (el.state) {
      subscribe_shiny_leptos_checkbox_group_buttons(el.state, callback);
    }
  }

  getValue(el: HTMLElement): any {
    if (!el.state) this.initialize(el);
    return get_shiny_leptos_checkbox_group_buttons_value(el.state);
  }

  setValue(el: HTMLElement, value: any): void {
    if (!el.state) this.initialize(el);
    update_shiny_leptos_checkbox_group_buttons(el.state, value);
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

window.Shiny.inputBindings.register(new ShinyLeptosCheckboxGroupButtonsBinding(), "ShinyLeptosCheckboxGroupButtonsBinding");
