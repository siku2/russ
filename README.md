# RUSS

## Features

- Type-safe. CSS is generated from Rust representations.
- Optimized for size. CSS is stored in a compact form instead of raw strings.
- Efficient. At runtime the CSS is only generated once.

## The Plan

1. Implement Rust equivalents for all basic CSS types (<https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Values_and_Units>).
2. Support for the most common CSS properties (<https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Properties_Reference>)
3. Add bindings for Yew. At this point the library starts becoming useable.
4. Add a `css_value!` macro which converts css values into their rust equivalents from step 1. 
   This should support dynamic expressions like `css_value! { {self.props.width}rem }`
5. Add support for all current css properties.
6. Add a macro which turns real css declaration bodies into the rust representation.
