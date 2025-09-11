# Columns

In Stwo, the computation trace is represented using multiple columns, each containing elements from the Mersenne prime field $\mathsf{M31}$. The columns are defined via the `Column<T>` trait, where `T` is typically `BaseField` (an alias for `M31`).

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/mod.rs 45:64}}
```

The operations over a column such as bit reversal of elements is provided using the `ColumnOps<T>` trait, which also implements the type alias `Col<B, T>` to conveniently represent a column.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/mod.rs 37:42}}
```

```admonish
Stwo defines a `Backend` trait, with two main implementations: `CpuBackend` and `SimdBackend`. The `SimdBackend` offers optimized routines for hardware supporting SIMD instructions, while `CpuBackend` provides a straightforward reference implementation.

Each backend implements the `ColumnOps` trait. Here and in the following sections, we will describe the trait implementations for the `CpuBackend`.
```

The `ColumnOps<T>` trait is implemented for the `CpuBackend` as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/mod.rs 32:38}}
```
Here, `bit_reverse` performs a naive bit-reversal permutation on the `column`.

## Secure Field Columns

<!-- TODO: add figure to showing secure columns -->

An element of the secure field (`SecureField` = `QM31`) cannot be stored in a single `BaseField` column because it is a quartic extension of `M31`. Instead, each secure field element is represented by four base field coordinates and stored in four consecutive columns.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/secure_column.rs 11:13}}
```

Here, `SECURE_EXTENSION_DEGREE` is the extension degree of `QM31` i.e. 4. You can think of each row of the 4 columns containing a single element of the `SecureField`. Thus accessing an element by index reconstructs it from its base field coordinates, implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/secure_column.rs 21:23}}
```

Now that we know how columns are represented, we can explore their use in storing evaluations over the circle domain and in interpolating polynomials.
