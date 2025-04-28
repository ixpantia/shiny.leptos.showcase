#' @export
shiny_leptos_counter_input <- function(inputId, value = 0.0) {
  attach_pkg_deps(
    shiny::div(
      id = inputId,
      class = "shiny_leptos_showcase-shiny_leptos_counter_input-container",
      `data-initial-value` = value
    )
  )
}

#' @export
update_shiny_leptos_counter_input <- function(
  inputId,
  value = 0.0,
  session = shiny::getDefaultReactiveDomain()
) {
  session$sendInputMessage(inputId, list(value = value))
}
