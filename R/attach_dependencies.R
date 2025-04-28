get_pkg_version <- function() {
  utils::packageDescription("shiny.leptos.showcase")$Version
}

attach_pkg_deps <- function(...) {
  package_version <- get_pkg_version()

  dep <- htmltools::htmlDependency(
    src = "dist",
    script = list(src = "shiny.leptos.showcase.js", type = "module"),
    stylesheet = list(src = "style.css"),
    version = "1.0",
    package = "shiny.leptos.showcase",
    name = "shiny.leptos.showcase"
  )

  htmltools::tagList(...) |>
    htmltools::attachDependencies(dep, append = TRUE)
}
