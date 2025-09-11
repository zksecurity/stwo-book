# Interpolation

This section covers the implementation of the `interpolate` function step by step. The function takes as input a `CircleEvaluation` and a `TwiddleTree` and outputs a `CirclePoly`.

The circle FFT algorithm changes the order of the input. Since the `CircleEvaluation` is in `BitReversedOrder`, the output coefficients of the `CirclePoly` will be in `NaturalOrder`. The complete implementation of the `interpolate` function is shown below:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 19:72}}
```

The function includes hardcoded implementations for circle domains of small sizes for efficiency. Lets breakdown the function step by step.

1. **Compute Twiddles:** Given the `TwiddleTree`, the function computes the line twiddles and circle twiddles.
   - The `circle_twiddles` are twiddles required at the first layer where all points are projected to the $x$-axis. These correspond to the $y$-coordinate points in the half coset.
   - The `line_twiddles` are twiddles required to compute FFT at the subsequent recursive layers where the squaring map $\pi$ is used as the 2-to-1 map.

   > For the example of the [half coset](./twiddles.md#fig-half-coset), there are three FFT layers: one layer with the projection map $\pi_x$ and two recursive layers with the squaring map $\pi$. 
   > 
   > The `line_twiddles` for the two recursive layers are computed using the inverse twiddles $[a^{-1}, b^{-1}, \pi(a)^{-1}, 1]$. For the first recursive layer, the twiddles are the inverse of the $x$-coordinates: $[a^{-1}, b^{-1}]$. For the second recursive layer, the twiddles are the inverse of the square of the $x$-coordinates: $[\pi(a)^{-1}]$. Thus for this example, the `line_twiddles` are $[[a^{-1}, b^{-1}], [\pi(a)^{-1}]]$.

   The `circle_twiddles` are computed using the first recursive layer `line_twiddles`. They are equal to the inverse of the $y$-coordinates of the half coset.
   > For this example, they take the values $[b^{-1}, -b^{-1}, -a^{-1}, a^{-1}]$.

2. **Apply FFT Layers:** With the twiddles for each layer computed, the FFT algorithm is applied first using the projection map $\pi_x$ and `circle_twiddles`, then using the squaring map $\pi$ and the `line_twiddles`. This process uses two key functions:

   - **`fft_layer_loop`**: Applies butterfly operations across a specific layer of the FFT. The key inputs are the values array, layer parameters, twiddle factor, and the butterfly function to apply. It iterates through pairs of elements in the values array at the appropriate indices and applies the butterfly operation with the twiddle factor.

   - **`ibutterfly`**: Performs the inverse butterfly operation on a pair of elements from the values array. This is the fundamental operation that transforms the values during interpolation.

3. **Scale and Output:** Finally, the `values` are scaled by dividing by the domain size, and the `CirclePoly` is output.

This completes the description of the `interpolate` function. The `evaluate` function follows a similar approach, applying the same components in reverse order to convert from coefficient representation back to point-value representation. In the next section, we will explore the FFT basis underlying the Circle FFT algorithm.
