# Columns

In Stwo, the computation trace is represented using multiple columns, each containing elements from the Mersenne prime field \\( \mathsf{M31} \\). The columns are defined via the `Column<T>` trait, where `T` is typically `BaseField` (an alias for `M31`).

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/mod.rs 45:64}}
```

The operations over a column such as bit reversal of elements is provided using the `ColumnOps<T>` trait and they also implement the type alias `Col<B, T>` to conveniently represent a column.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/mod.rs 37:42}}
```

## Secure Field Columns

<!-- TODO: add figure to showing secure columns -->

An element of the secure field (`SecureField` = `QM31`) cannot be stored in a single `BaseField` column because it is a quartic extension of `M31`. Instead, each secure field element is represented by four base field coordinates and stored in four parallel columns.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/secure_column.rs 8:13}}
```

Here, `SECURE_EXTENSION_DEGREE` is the extension degree of `QM31` i.e. 4. You can think of each row of the 4 columns containing a single element of the `SecureField`. Thus accessing an element by index reconstructs it from its base field coordinates, implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/secure_column.rs 21:23}}
```

Now that we know how columns are represented, we can explore their use in storing evaluations over the circle domain and in interpolating polynomials.