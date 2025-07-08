# stwo-book

This is the repository for the Stwo Book, a book about Starkware's [Stwo prover](https://github.com/starkware-libs/stwo).

## How to build the book locally

Install `mdbook` following the instructions [here](https://rust-lang.github.io/mdBook/guide/installation.html).

Then, setup `mdbook-admonish` by first installing it with the following command:

```bash
cargo install mdbook-admonish
```

and then running the following command at the root of this repository:

```bash
mdbook-admonish install .
```

Then, you can run the following command to serve the book locally:

```bash
mdbook serve
```
