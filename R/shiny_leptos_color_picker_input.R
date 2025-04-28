#' Leptos Color Picker Input
#'
#' Creates a color picker input widget using Leptos (Rust/WASM).
#'
#' @param inputId The `input` slot that will be used to access the value.
#' @param value Initial color value (hex string, e.g., "#RRGGBB"). Defaults to "#000000".
#' @param label Optional label for the input.
#' @param width The width of the input, e.g. `'100px'`, or `'100%'`;
#'   see \code{\link[shiny]{validateCssUnit}}.
#'
#' @return An HTML tag object that can be included in a Shiny UI definition.
#' @export
#' @importFrom htmltools tags validateCssUnit tagList div span
#' @importFrom shiny restoreInput
shiny_leptos_color_picker_input <- function(inputId, label = NULL, value = "#000000", width = NULL) {
  value <- shiny::restoreInput(id = inputId, default = value)
  if (!startsWith(value, "#") || nchar(value) != 7) {
      warning("Initial color value should be a 7-character hex string like '#RRGGBB'. Using default '#000000'.")
      value <- "#000000"
  }

  style <- NULL
  if (!is.null(width)) {
    style <- paste0("width: ", htmltools::validateCssUnit(width), ";")
  }

  # Use htmltools::tags for better structure if label exists
  input_tag <- htmltools::div(
      id = inputId,
      class = "shiny_leptos_showcase-shiny_leptos_color_picker_input-container form-group shiny-input-container",
      style = style,
      `data-initial-value` = value
      # The Leptos component will render the <input type='color'>
  )

  if (!is.null(label)) {
      res <- htmltools::tagList(
          htmltools::tags$label(label, class = "control-label", `for` = inputId),
          input_tag
      )
  } else {
      res <- input_tag
  }

  attach_pkg_deps(res)
}

#' Update Leptos Color Picker Input
#'
#' Changes the value of a color picker input on the client side.
#'
#' @param inputId The ID of the input object.
#' @param value The new color value (hex string, e.g., "#RRGGBB").
#' @param session The Shiny session object, defaults to the current reactive domain.
#' @export
update_shiny_leptos_color_picker_input <- function(
  inputId,
  value,
  session = shiny::getDefaultReactiveDomain()
) {
  if (!startsWith(value, "#") || nchar(value) != 7) {
      warning("Color value should be a 7-character hex string like '#RRGGBB'.")
      # Optionally return or stop here, or proceed cautiously
  }
  session$sendInputMessage(inputId, list(value = value))
}
