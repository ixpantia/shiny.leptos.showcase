library(shiny)
library(shiny.leptos.showcase)
# devtools::load_all()
library(bslib)

ui <- page_fluid(
  theme = bs_theme(version = 5),
  fluidRow(
    column(
      4,
      wellPanel(
        h3("Counter Input"),
        shiny_leptos_counter_input(
          "counter",
          value = 0.0
        ),
        hr(),
        p("Current Value:", style = "color:#888888;"),
        verbatimTextOutput("counterOut"),
        a(
          "See Code",
          class = "btn btn-primary btn-md",
          href = "https://github.com/ixpantia/shiny.leptos.showcase/blob/main/srcrs/src/shiny_leptos_counter_input.rs"
        )
      )
    ),

    column(
      4,
      wellPanel(
        h3("Text Input"),
        shiny_leptos_text_input(
          "text",
          value = "Hello From Leptos!"
        ),
        hr(),
        p("Current Value:", style = "color:#888888;"),
        verbatimTextOutput("textOut"),
        a(
          "See Code",
          class = "btn btn-primary btn-md",
          href = "https://github.com/ixpantia/shiny.leptos.showcase/blob/main/srcrs/src/shiny_leptos_text_input.rs"
        )
      )
    ),

    column(
      4,
      wellPanel(
        h3("Group Buttons Input"),
        shiny_leptos_checkbox_group_buttons(
          "checkGroup",
          value = "option_1",
          options = list(
            options = c("option_1", "option_2", "option_3"),
            labels = c("Option 1", "Option 2", "Option 3")
          )
        ),
        hr(),
        p("Current Values:", style = "color:#888888;"),
        verbatimTextOutput("checkGroupOut"),
        a(
          "See Code",
          class = "btn btn-primary btn-md",
          href = "https://github.com/ixpantia/shiny.leptos.showcase/blob/main/srcrs/src/shiny_leptos_checkbox_group_buttons.rs"
        )
      )
    )
  ),
  br(),
  fluidRow(

    column(
      4,
      wellPanel(
        h3("Dynamic Items Input"),
        shiny_leptos_dynamic_items_input(
          "dynamicList",
          value = c("Apple", "Banana"), # Initial items as a character vector
          width = "100%"
        ),
        hr(),
        p("Current Value:", style = "color:#888888;"),
        verbatimTextOutput("dynamicListOut"),
        a(
          "See Code",
          class = "btn btn-primary btn-md",
          href = "https://github.com/ixpantia/shiny.leptos.showcase/blob/main/srcrs/src/shiny_leptos_dynamic_items_input.rs"
        )
      )

    ),

    column(
      4,
      wellPanel(
        h3("Color Picker Input"),
        shiny_leptos_color_picker_input(
          "colorPicker",
          label = "Select Color:",
          value = "#e66465", # A nice default coral color
          width = "100px"    # Color pickers are usually small
        ),
        hr(),
        p("Current Value:", style = "color:#888888;"),
        verbatimTextOutput("colorPickerOut"),
        a(
          "See Code",
          class = "btn btn-primary btn-md",
          style="margin-top: 15px;",
          href = "https://github.com/ixpantia/shiny.leptos.showcase/blob/main/srcrs/src/shiny_leptos_color_picker_input.rs" # Link to code if available
        )
      )
    )
  )
)

server <- function(input, output, session) {
  output$counterOut <- renderPrint({
    input$counter
  })

  output$textOut <- renderPrint({
    input$text
  })

  output$checkGroupOut <- renderPrint({
    input$checkGroup
  })

  output$dynamicListOut <- renderPrint({
    input$dynamicList
  })

  output$colorPickerOut <- renderPrint({
    input$colorPicker
  })

}

shinyApp(ui = ui, server = server)
