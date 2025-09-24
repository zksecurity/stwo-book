# Prover Components

The `ComponentProvers` struct is similar to the `Components` struct but implements additional functions required by the prover, such as computing the composition polynomial. It is a collection of prover components as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/air/component_prover.rs 33:36}}
```

Here, `components` is a collection of objects that implement the `ComponentProver` trait. The `ComponentProver` trait is a wrapper around the `Component` trait with an additional function shown as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/air/component_prover.rs 13:21}}
```

We can convert the `ComponentProvers` into a `Components` struct as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/air/component_prover.rs 39:48}}
```

The main function defined on the `ComponentProvers` struct to compute the composition polynomial is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/air/component_prover.rs 49:64}}
```

Let us examine the above function for our [example AIR containing two components](./overview.md#air-to-composition-polynomial). It takes the following three inputs:
- `&self`: This is the `ComponentProvers` on which the function is called.
- `random_coeff`: This is an element from the `SecureField` (i.e. $\mathsf{QM31}$). In our example, this is represented as $\gamma$.
- `trace`: The `Trace` struct which contains all the polynomials that make up the entire trace including all the components. For efficiency, it stores each polynomial in both coefficients and evaluations form.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/air/component_prover.rs 26:31}}
```

Now let us examine the body of the function. First, we compute `total_constraints` and initialize an `accumulator`. The `total_constraints` determine the number of powers of $\gamma$ (`random_coeff`) required for the random linear combination.

For each component, we call `evaluate_constraint_quotients_on_domain`, which computes and accumulates the evaluations of that component's quotients on their respective evaluation domains within the accumulator. For the $0$th component, we add the evaluations of the quotient $q_0$ over its evaluation domain $D_{n_0 + \beta}$ to the `accumulator`. Similarly, for the $1$st component, we add the evaluations of the quotient $q_1$ over its evaluation domain $D_{n_1 + \beta}$ to the `accumulator`.

After adding all component quotient evaluations to the accumulator, we call the `finalize()` function, which:
1. Combines the accumulated evaluations at different domain sizes to compute the evaluations of the quotient composition polynomial $q$ over the domain $D_{n + \beta}$ where $n = \max{(n_1, n_2)}$.
2. Interpolates $q$ over $D_{n + \beta}$ using [circle FFT](../circle-fft/index.md) to convert it into coefficient representation.

Note that the output is a [`SecureCirclePoly`](../circle-polynomials/secure-evals-and-poly.md#secure-circle-polynomials) since the evaluations of $q$ are in the secure field $\mathsf{QM31}$ (as $\gamma$ is randomly sampled from $\mathsf{QM31}$).