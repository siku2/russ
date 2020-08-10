fn test() {
    // css! macro which creates a Css type. In this case it should expand to something like `Length::Em(Integer(5))`
    let font_size = css!(5rem);

    // the styles! macro creates a stylesheet. id selectors like "main" are dynamically turned into unique classes.
    // The return value is a struct with a field for each selector. The value of such a field implements `Into<Classes>`.
    let style = styles! {
      main {
        background: white;
        font-size: { font_size };
      }

      main:hover {
        background: black;
      }
    };

    html! {
      <div class=style.main>
        { "Hello World!" }
      </div>
    }
}
