#' @export
shiny_leptos_text_input <- function(inputId, value = "", label = NULL) {
  attach_pkg_deps(
    shiny::div(
      id = inputId,
      class = "shiny_leptos_showcase-shiny_leptos_text_input-container",
      `data-initial-value` = value,
      `data-label` = label,
    )
  )
}

#' @export
update_shiny_leptos_text_input <- function(
  inputId,
  value,
  session = shiny::getDefaultReactiveDomain()
) {
  session$sendInputMessage(inputId, list(value = value))
}
