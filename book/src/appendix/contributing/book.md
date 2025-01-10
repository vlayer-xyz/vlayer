# Contributing to vlayer Book

## Prerequisites
Ensure you have [Rust](https://www.rust-lang.org/learn) and the [Cargo package manager](https://doc.rust-lang.org/cargo/) installed:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installing Rust, install the required dependencies:
- `mdbook`: A command-line tool for creating books with Markdown.
- `mdbook-mermaid`: A preprocessor for compiling Mermaid diagrams.
- `mdbook-tabs`: A plugin for adding tab functionality to the book.

```sh
cargo install mdbook mdbook-mermaid mdbook-tabs
```

## Development

The book's source is in the vlayer monorepo. To start the development server, navigate to the `book/` directory and run:
```sh
mdbook serve
```

Whenever you update the book's source, the preview will automatically refresh. Access the preview at `http://localhost:3000`.

## Building

To build the book, navigate to the `book/` directory and run:
```sh
mdbook build
```

## Building

Rust allows you to set granular log levels for different crates using [RUST_LOG](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). To debug a specific crate, you can set its log level to `debug`. For example:
```sh
RUST_LOG=info,call_engine=debug ./target/debug/call_server
```

The static HTML output will be generated in the `book/book` directory. You can use this output to preview the book locally or deploy it to a static site hosting service.
