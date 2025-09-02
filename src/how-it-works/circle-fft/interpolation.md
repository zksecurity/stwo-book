# Interpolation

In this section, we will go over the implementation of the `interpolate` function step by step. Let us first look at the function signature:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 19:22}}
```

The function takes as input a `CircleEvaluation` and a `TwiddleTree` and outputs a `CirclePoly`. As we have seen, the circle FFT algorithm changes the order of the input. Since here `CircleEvaluation` is in `BitReversedOrder` the output coefficients of the `CirclePoly` will be in `NaturalOrder`.

Now given the `TwiddleTree`, the function first computes the line twiddles and circle twiddles.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 53:54}}
```

The `circle_twiddles` are twiddles required at the first layer where all points are projected to the $ x $-axis. As we have discussed, the are the $ y $-coordinate points in the half coset. The `line_twiddles` are the twiddles required to compute FFT at the subsequent recursive layers where we use the squaring map $ \pi $ as the 2-to-1 map.

For the example of the [half coset](./twiddles.md#fig-half-coset), there will be three FFT layers: one layer with projection map $ \pi_x $ and two recursive layers with the squaring map $ \pi $. 

We first compute the `line_twiddles` for the two recursive layers using the inverse twiddles $ [a^{-1}, b^{-1}, \pi(a)^{-1}, 1] $. For the first recursive layer, the twiddles will just be inverse of the $ x $-coordinates i.e. $ [a^{-1}, b^{-1}] $ and for the second recursive layer the twiddles will be inverse of the square of the x-coordinates i.e. $ [\pi(a)^{-1}] $. Thus for our example, the `line_twiddles` will be $ [[a^{-1}, b^{-1}], [\pi(a)^{-1}]] $.

We can compute the `circle_twiddles` using the first recursive layer `line_twiddles`. They will be equal to the inverse of the $ y $-coordinates of the half coset. For our example, they will take the values $ [b^{-1}, -b^{-1}, -a^{-1}, a^{-1}] $.

Now that we have the twiddles for each layer, we will apply the FFT algorithm first using the projection map $ \pi_x $ and `circle_twiddles` and then using the squaring map $ \pi $ and the `line_twiddles`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 56:63}}
```

Finally, we scale the `values` and output the `CirclePoly`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 65:71}}
```

Here is the complete implementation of the `interpolate` function:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 19:72}}
```
Note that we are hardcoding the function for lower size of circle domain for efficiency.

This completes our description of the `interpolation` function. The `evaluate` function follows the same approach, using the same components in reverse, so we will not cover it here. In the next section, we will discuss some theoretical notes on the FFT basis.
