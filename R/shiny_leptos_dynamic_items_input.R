#' Dynamic List Input Widget
#'
#' Creates a dynamic list input widget using Leptos (Rust/WASM). Users can
#' add items using a text input and remove items using buttons.
#'
#' @param inputId The `input` slot that will be used to access the value.
#' @param value Initial value for the list. Should be a character vector or list.
#'              Defaults to an empty list.
#' @param width The width of the input, e.g. `'400px'`, or `'100%'`;
#'   see \code{\link[shiny]{validateCssUnit}}.
#'
#' @return An HTML tag object that can be included in a Shiny UI definition.
#'
#' @export
#' @importFrom htmltools tags validateCssUnit tagList attachDependencies htmlDependency
#' @importFrom shiny div getDefaultReactiveDomain
#' @importFrom jsonlite toJSON
#'
#' @examples
#' if (interactive()) {
#'   library(shiny)
#'   library(shiny.leptos.showcase)
#'
#'   ui <- fluidPage(
#'     h3("Dynamic Items Input"),
#'     shiny_leptos_dynamic_items_input(
#'       "dynamicList",
#'       value = list("Initial Item 1", "Initial Item 2"),
#'       width = "400px"
#'     ),
#'     hr(),
#'     h4("Current Value:"),
#'     verbatimTextOutput("dynamicListOut")
#'   )
#'
#'   server <- function(input, output, session) {
#'     output$dynamicListOut <- renderPrint({
#'       input$dynamicList
#'     })
#'   }
#'
#'   shinyApp(ui, server)
#' }
shiny_leptos_dynamic_items_input <- function(inputId, value = list(), width = NULL) {

  # Ensure value is NULL or a list/vector that can be JSONified into an array
  if (!is.null(value) && !is.vector(value) && !is.list(value)) {
    stop("'value' must be a vector, a list, or NULL.")
  }
  if (is.null(value)) {
      value <- list() # Represent NULL as an empty list for JSON
  }

  # Convert the R list/vector to a JSON array string
  # auto_unbox = TRUE helps ensure simple vectors become JSON arrays, not single values
  value_json <- jsonlite::toJSON(value, auto_unbox = TRUE)

  style <- NULL
  if (!is.null(width)) {
    style <- paste0("width: ", htmltools::validateCssUnit(width), ";")
  }

  # Attach package dependencies (JS/CSS)
  attach_pkg_deps(
    shiny::div(
      id = inputId,
      class = "shiny_leptos_showcase-shiny_leptos_dynamic_items_input-container form-group shiny-input-container",
      style = style,
      # Pass the initial value as a JSON string
      `data-initial-value` = value_json
      # Add other data attributes if needed (e.g., placeholder text)
      # `data-placeholder` = "Add item..."
    )
  )
}

#' Update Dynamic List Input Widget
#'
#' Changes the value of a dynamic list input widget on the client side.
#'
#' @param inputId The ID of the input object.
#' @param value The new value for the list. Should be a character vector or list.
#' @param session The Shiny session object, defaults to the current reactive domain.
#'
#' @export
update_shiny_leptos_dynamic_items_input <- function(
  inputId,
  value,
  session = shiny::getDefaultReactiveDomain()
) {
  # Ensure value is NULL or a list/vector
  if (!is.null(value) && !is.vector(value) && !is.list(value)) {
    stop("'value' must be a vector, a list, or NULL.")
  }
   if (is.null(value)) {
      value <- list() # Represent NULL as an empty list for the message
  }

  # Send the new value to the client. Shiny handles JSON conversion.
  session$sendInputMessage(inputId, list(value = value))
}
