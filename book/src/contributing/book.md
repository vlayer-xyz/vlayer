# Contributing to vlayer book

## Prerequisites

To build this book, you will need `mdbook` installed:
```sh
cargo install mdbook
```

To compile diagrams in the book, you need to install [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) preprocessor:
```sh
cargo install mdbook-mermaid
```

## Building

Book source is available in the vlayer monorepo. To build the book navigate to `book/` and type:
```
mdbook serve
```

Now, the preview of the book is available at `http://localhost:3000/`.
