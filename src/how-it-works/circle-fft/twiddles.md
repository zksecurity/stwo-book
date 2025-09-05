# Twiddles

This section provides a detailed look at how twiddles are computed and stored in Stwo.

```admonish
Stwo defines a `Backend` trait, with two main implementations: `CpuBackend` and `SimdBackend`. The `SimdBackend` offers optimized routines for hardware supporting SIMD instructions, while `CpuBackend` provides a straightforward reference implementation.

Each backend implements the `PolyOps` trait, which provides core polynomial operations such as interpolation, evaluation and twiddle precomputation. Here and in the following sections on Circle FFT, we will explore how these functions are implemented for the `CpuBackend`.
```

## Twiddle Tree

The twiddles are stored using the `TwiddleTree` struct implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/poly/twiddles.rs 4:14}}
```

For `CpuBackend`, `B::Twiddles` is a vector of `BaseField` elements. Here, `root_coset` is the half coset $q + \langle g_{n-1} \rangle$ of our circle domain $D_n$. As we have seen in the earlier section, for interpolation we divide by twiddles or multiply by inverse twiddles. In the evaluation algorithm, we multiply by twiddles. Thus we store both `twiddles` and their inverses `itwiddles`.

Now we will understand how the twiddle tree is computed using an example. Consider the following half coset $q + \langle g_2 \rangle$ of a circle domain $D_3$.

<div style="text-align: center;">
    <figure id="fig-half-coset" style="display: inline-block;">
    <img src="../figures/half-coset.svg" width="800px" style="border-radius: 8px;" />
        <figcaption><span style="font-size: 0.9em">Figure: Half Coset of size 4</span></figcaption>
    </figure>
</div>

The `TwiddleTree` is constructed by the `precompute_twiddles` function, which takes the half coset as input and produces the twiddles needed to perform FFT over the corresponding circle domain. It is implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 138:169}}
```

As shown above, it first computes `twiddles` using the function `slow_precompute_twiddles` then computes their inverses `itwiddles` using batch inversion and finally stores then in the `TwiddleTree` along with the `root_coset` which is the input half coset.

Now let us look into the function `slow_precompute_twiddles`.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/circle.rs 172:189}}
```

The above function computes the twiddles required to compute FFT over the line (i.e. the recursive layer FFT, after projecting to the $x$-axis). For the example in the figure with the half coset points $[(a, b), (b, -a), (-a, -b), (-b, a)]$, this function will output $[a, b, \pi(a), 1]$.

Thus for the half coset in the figure, the `precompute_twiddles` will output `TwiddleTree` with `twiddles` as $[a, b, \pi(a), 1]$ and `itwiddles` as $[a^{-1}, b^{-1}, \pi(a)^{-1}, 1]$. The `twiddles` will be used for evaluation algorithm and `itwiddles` will be used for the interpolation algorithm.

In the next section, we will bring together everything we've covered so far on Circle FFT to examine the implementation of the interpolation algorithm.
