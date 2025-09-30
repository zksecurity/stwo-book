# Static Lookups

In the previous section, we showed how to create a preprocessed trace. In this section, we will introduce the concept of an interaction trace, and use it with the preprocessed trace to implement **static lookups**.

Let's start with a brief introduction to lookups. A lookup is a way to connect values from one part of the table to another part of the table. A simple example is when we want to copy values across parts of the table. At first glance, this seems feasible using a constraint. For example, we can copy $col_1$ values to $col_2$ by creating a constraint that $col_1 - col_2$ is equal to $0$. The limitation with this approach, however, is that the same constraint needs to be satisfied over every row in the columns. In other words, we can only check that $col_2$ is an exact copy of $col_1$:

$$
col_1[i] = col_2[i] \quad \forall\, i
$$

But what if we want to check that $col_2$ is a copy of $col_1$ regardless of the order of the values? This can be done by comparing that the grand product of the random linear combinations of all values in $col_1$ is equal to the grand product of the random linear combinations of all values in $col_2$:

$$
\prod_{i=0}^{n-1} (X - col_1[i]) = \prod_{i=0}^{n-1} (X - col_2[i])
$$

where $X$ is a random value from the verifier.

By taking the logarithmic derivative of each side of the equation, we can rewrite it:

$$
\sum_{i=0}^{n-1} \frac{1}{X-col_1[i]} = \sum_{i=0}^{n-1} \frac{1}{X-col_2[i]}
$$

We can go further and allow each of the original values to be copied a different number of times. This is supported by modifying the check to the following:

$$
\sum_{i=0}^{n-1} \frac{1}{X-col_1[i]} = \sum_{i=0}^{n-1} \frac{m_i}{X-col_2[i]}
$$

where $m_i$ represents the multiplicity, or the number of times $col_1[i]$ appears in $col_2$.

In Stwo, these fractions (which we will hereafter refer to as _LogUp fractions_) are stored in a special type of trace called an _interaction trace_. An interaction trace is used to contain values that involve interaction between the prover and the verifier. As mentioned above, a LogUp fraction requires a random value $X$ from the verifier, which is why it is stored in an interaction trace.

## Range-check AIR

We will now walk through the implementation of a static lookup, which is a lookup where the values that are being looked up are static, i.e. part of the preprocessed trace. Specifically, we will implement a **range-check AIR**, which checks that a certain value is within a given range. This is especially useful for frameworks like Stwo that use finite fields because it allows checking for underflow and overflow.

A range-check checks that all values in a column are within a certain range. For example, as in [Figure 1](#fig-range-check), we can check that all values in the range-checked columns are between 0 and 3. We do this by first creating a multiplicity column that counts the number of times each value in the preprocessed trace appears in the range-checked columns.

Then, we create two LogUp columns as part of the interaction trace. The first column contains in each row a fraction with numerator equal to the multiplicity and denominator equal to the random linear combination of the value in the range column. For example, for row 1, the fraction should be $\frac{2}{X-0}$, where $X$ is a random value. The second column contains batches of fractions where the denominator of each fraction is the random linear combination of the value in the range-checked column. Note that the numerator of each fraction is always -1, i.e. we apply a negation, because we want the sum of the first column to be equal to the sum of the second column.

<figure id="fig-range-check" style="text-align: center;">
    <img src="./range-check.png" width="100%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 1: Range-check lookup</span></center></figcaption>
</figure>

If we add all the fractions in the two columns together, we get 0. This means that the verifier will be convinced with high probability that the values in the range-checked columns are a subset of the values in the range column.

## Implementation

Now let's move on to the implementation where we create a 4-bit range-check AIR. We do this by creating a preprocessed trace column with the integers $[0, 16)$, then using a lookup to force the values in the original trace columns to lie in the values of the preprocessed column.

```rust,ignore
{{#include ../../../stwo-examples/examples/static_lookups.rs:range_check_column}}
```

First, we need to create the range-check column as a preprocessed column. This should look familiar to the code from the previous section.

```rust,ignore
{{#include ../../../stwo-examples/examples/static_lookups.rs:gen_trace}}
```

Next, we create the original trace columns. The first two columns are random values in the range $[0, 15]$, and the third column contains the counts of the values in the range-check column.

```rust,ignore
{{#include ../../../stwo-examples/examples/static_lookups.rs:gen_logup_trace}}

{{#include ../../../stwo-examples/examples/static_lookups.rs:main_start}}
    ...
{{#include ../../../stwo-examples/examples/static_lookups.rs:logup_start}}
    ...
{{#include ../../../stwo-examples/examples/static_lookups.rs:main_end}}
```

Now we need to create the LogUp columns.

First, note that we are creating a `SmallerThan16Elements` instance using the macro `relation!`. This macro creates an API for performing random linear combinations. Under the hood, it creates two random values $z, \alpha$ that can create a random linear combination of an arbitrary number of elements. In our case, we only need to combine one value (value in $[0,15]$), which is why we pass in `1` to the macro.

Inside `gen_logup_trace`, we create a `LogupTraceGenerator` instance. This is a helper class that allows us to create LogUp columns. Every time we create a new column, we need to call `new_col()` on the `LogupTraceGenerator` instance.

You may notice that we are iterating over `BaseColumn` in chunks of 16, or `1 << LOG_N_LANES` values. This is because we are using the `SimdBackend`, which runs 16 lanes simultaneously, so we need to preserve this structure. The `Packed` in `PackedSecureField` means that it packs 16 values into a single value.

You may also notice that we are using a `SecureField` instead of just the `Field`. This is because the random value we created by `SmallerThan16Elements` lies in the degree-4 extension field $\mathbb{F}_{p^4}$. This is necessary for the security of the protocol and interested readers can refer to the [Mersenne Primes](../../how-it-works/mersenne-prime.md) section for more details.

Once we set the fractions for each `simd_row`, we need to call `finalize_col()` to finalize the column. This process modifies the LogUp columns from individual fractions to cumulative sums of the fractions as shown in [Figure 2](#fig-finalize-col).

<figure id="fig-finalize-col" style="text-align: center;">
    <img src="./finalize-col.png" width="80%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 2: Finalizing each LogUp column</span></center></figcaption>
</figure>

Finally, we need to call `finalize_last()` on the `LogupTraceGenerator` instance to finalize the LogUp columns, which will return the LogUp columns as well as the sum of the fractions in the LogUp columns.

```rust,ignore
{{#include ../../../stwo-examples/examples/static_lookups.rs:test_eval}}
```

The last piece of the puzzle is to create the constraints. We use the same `TestEval` struct as in the previous sections, but the `evaluate` function will look slightly different. Instead of calling `add_constraint` on the `EvalAtRow` instance, we will call `add_to_relation`, which recreates the fractions that we added in the LogUp columns using values in the range-check, lookup, and multiplicity columns.

Once we add the fractions as constraints, we call the `finalize_logup_batched` function, which indicates how we want to batch the fractions. In our case, we added 3 fractions but want to create batches where the last two fractions are batched together, so we pass in `&vec![0, 1, 1]`.

```rust,ignore
{{#include ../../../stwo-examples/examples/static_lookups.rs:verify}}
```

When we verify the proof, as promised, we check that the `claimed_sum`, which is the sum of the fractions in the LogUp columns, is 0.

And that's it! We have successfully created a static lookup for a range-check.

```admonish
**How many fractions can we batch together?**

This depends on how we set the `max_constraint_log_degree_bound` function, as discussed in this [note](../writing-a-simple-air/constraints-over-trace-polynomials.md#max_constraint_log_degree_bound). More specifically, we can batch up to exactly the blowup factor.

e.g.

- `self.log_size + 1` -> 2 fractions
- `self.log_size + 2` -> 4 fractions
- `self.log_size + 3` -> 8 fractions
- `self.log_size + 4` -> 16 fractions
- ...
```

```admonish
Note that unlike what [Figure 1](#fig-range-check) shows, the size of the range column and the range-checked columns do not have to be the same. As we will learn in the [Components](../components/index.md) section, we can create separate components for the range-check and the range-checked columns to support such cases.
```

$$
$$
