# RUSS

A _hopefully_ soon-to-be css-in-Rust solution.

## Goals

- Type-safe. CSS is generated from Rust representations.
- Optimized for size. CSS is stored in a compact form instead of raw strings.
- Efficient. At runtime the CSS is only generated once.

## The Plan

1. ~~Implement Rust equivalents for all basic CSS types (<https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Values_and_Units>).~~
2. ~~Add a macro that parses CSS' [value definition syntax](https://www.w3.org/TR/css-values-3/#value-defs) (VDS)~~
3. Generate Rust code from the VDS.
   This should generate a rust structure that can represent valid values of the VDS and also a macro representation that parses values into said structure.
4. Implement the the most common CSS properties.
5. Add bindings for Yew. At this point the library starts becoming useable.
6. Add the remaining properties.
