# Hello (ZK) World

Let's first set up a Rust project with Stwo.

```bash
$ cargo new stwo-example
```

We need to specify the nightly Rust compiler to use Stwo.

```bash
$ echo -e "[toolchain]\nchannel = \"nightly-2025-01-02\"" > rust-toolchain.toml
```

Now let's edit the `Cargo.toml` file as follows:

```rust,ignore
{{#include ../../../stwo-examples/Cargo.toml}}
```

We are all set!
