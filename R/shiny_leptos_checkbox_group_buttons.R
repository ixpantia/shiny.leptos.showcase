#' @export
shiny_leptos_checkbox_group_buttons <- function(inputId, value = NULL, options) {

  if (is.null(options)) {
    stop("options must be provided")
  }

  if (!is.list(options)) {
    stop("options must be a list")
  }

  if (is.null(options$options)) {
    stop("options$options must be provided")
  }

  if (is.null(options$labels)) {
    options$labels <- options$options
  }

  if (!is.character(options$labels)) {
    stop("options$labels must be a character vector")
  }

  if (!is.character(options$options)) {
    stop("options$options must be a character vector")
  }

  attach_pkg_deps(
    shiny::div(
      id = inputId,
      class = "shiny_leptos_showcase-shiny_leptos_checkbox_group_buttons-container",
      `data-initial-value` = value,
      `data-options` = jsonlite::toJSON(options, auto_unbox = FALSE)
    )
  )
}

#' @export
update_shiny_leptos_checkbox_group_buttons <- function(
  inputId,
  value,
  session = shiny::getDefaultReactiveDomain()
) {
  session$sendInputMessage(inputId, list(value = value))
}
