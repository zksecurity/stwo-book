# Local Row Constraints

Until now, we have only considered constraints that apply over values in a single row. But what if we want to express constraints over multiple adjacent rows? For example, we may want to ensure that the difference between the values in two adjacent rows is always the same.

Turns out we can implement this as an AIR constraint, as long as the same constraints are applied to all rows. We will build upon the example in the previous section, where we created two columns and proved that they are permutations of each other by asserting that the second column looks up all values in the first column exactly once.

Here, we will create two columns and prove that not only are they permutations of each other, but also that the second row is a sorted version of the first row. Since the sorted column will contain in order the values $[0,num\_rows)$, this is equivalent to asserting that **the difference between every current row and the previous row is $1$**.

We will implement this in three iterations, fixing a different issue in each iteration.

## First Try

```rust,ignore
{{#include ../../../stwo-examples/examples/local_row_constraints_fails_1.rs:evaluate}}
```

The logic for creating the trace and LogUp columns is basically the same as in the previous section (except that one of the columns is now sorted), so we omit them for brevity.

Another change is in the `evaluate` function, where we call `eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [-1, 0])` instead of `eval.next_trace_mask()`. The function `next_trace_mask()` is a wrapper for `next_interaction_mask(ORIGINAL_TRACE_IDX, [0])`, where the first parameter specifies which part of the trace to retrieve values from (see [this figure](../static-lookups/index.md#fig-range-check) for an example of the different parts of a trace). Since we want to retrieve values from the original trace, we set the value of the first parameter to `ORIGINAL_TRACE_IDX`. Next, the second parameter indicates the row offset of the value we want to retrieve. Since we want to retrieve both the previous and current row values for the sorted column, we set the value of the second parameter to `[-1, 0]`.

Once we have these values, we can now assert that the difference between the current and previous row is always `1` with the constraint: `E::F::one() - (sorted_col_curr_row.clone() - sorted_col_prev_row.clone())`.

```admonish question
But this will fail with a `ConstraintsNotSatisfied` error, can you see why? (You can try running it yourself [here](https://github.com/zksecurity/stwo-book/blob/main/stwo-examples/examples/local_row_constraints_fails_1.rs))
```

## Second Try

The issue was that when calling `evaluate` on the first row of our trace, the previous row value wraps around to the last row because there are no negative indices.

This means that in our example, we are expecting the `0 - 15 = 1` constraint to hold, which is clearly not true.

To fix this, we can use the `IsFirstColumn` preprocessed column that we created in the [Preprocessed Trace](../preprocessed-trace/index.md) section. So we will copy over the same code for creating the preprocessed column and modify our new constraint as follows:

```rust,ignore
{{#include ../../../stwo-examples/examples/local_row_constraints_fails_2.rs:constraint}}
```

Now, we have a constraint that is disabled for the first row, which is exactly what we want.

Still, however, this will fail with the same `ConstraintsNotSatisfied` error. (You can run it [here](https://github.com/zksecurity/stwo-book/blob/main/stwo-examples/examples/local_row_constraints_fails_2.rs))

## Third Try

So when we were creating `CircleEvaluation` instances from our `BaseColumn` instances, the order of the elements that we were creating it with was actually not the order that Stwo understands it to be. Instead, it assumes that the values are in the bit-reversed, circle domain order. It's not important to understand what this order is, specifically, but this does mean that when Stwo tries to find the `-1` offset when calling `evaluate`, it will find the previous value assuming that it's in a different order. This means that when we create a `CircleEvaluation` instance, we need to convert it to a bit-reversed circle domain order.

Thus, every time we create a `CircleEvaluation` instance, we need to convert the order of the values in the `BaseColumn` beforehand.

```rust,ignore
{{#include ../../../stwo-examples/examples/local_row_constraints.rs:is_first_column_impl_start}}
    ...
{{#include ../../../stwo-examples/examples/local_row_constraints.rs:is_first_column}}
    ...
{{#include ../../../stwo-examples/examples/local_row_constraints.rs:is_first_column_impl_end}}

{{#include ../../../stwo-examples/examples/local_row_constraints.rs:gen_trace}}
```

And voilà, we have successfully implemented the constraint. You can run it [here](https://github.com/zksecurity/stwo-book/blob/main/stwo-examples/examples/local_row_constraints.rs).

```admonish summary
Things to consider when implementing constraints over multiple rows:
1. Change the order of elements in `BaseColumn` in-place via `bit_reverse_coset_to_circle_domain_order` before creating a `CircleEvaluation` instance. This is required because Stwo assumes that the values are in the bit-reversed, circle domain order.
2. For the first row, the 'previous' row is the last row of the trace, so you may need to disable the constraint for the first row. This is typically done by using a preprocessed column.
```
