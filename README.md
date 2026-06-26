# Rust for R developers @ Cascadia R Conference

Materials for two hands-on workshops, rendered together as a single Quarto book.

📖 **Read it online:** <https://josiahparry.github.io/rust-for-r-devs>

## The workshops

1. **Intro to Rust for R Developers** — Rust fundamentals from the ground up:
   types, control flow, collections, ownership, iterators, structs, enums, and
   error handling.
2. **Building Rust-based R Packages** — putting that Rust to work by building a
   real R package, `{rurls}`, that wraps the [`url`](https://docs.rs/url) crate
   with [extendr](https://extendr.github.io/).

Slides for each live in `slides/` (`p1.qmd` and `p2.qmd`).

## Solutions

- **Workshop 1** solutions are in `intro-to-rust/examples/` — one `.rs` file per
  chapter.
- **Workshop 2** solutions are the R package in `rurls/`.
