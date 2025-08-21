In this section we will describe the Circle FFT and go over the implementation of functions `interpolate` and `evaluate`.

```admonish
The implementation of functions `interpolate` and `evaluate` depend on the choice of backend. Stwo implements a trait called `Backend` which is currently implemented for two backends: `CpuBackend` and `SimdBackend`. The `SimdBackend` backend offers optimized implementations for chips which support the SIMD instructions. But for the purpose of understanding the inner working of Stwo we will focus on the implementations in the `CpuBackend` since it has simpler implementations. Exploring the optimizations in `SimdBackend` is left as an exercise to the reader.

Each backend implements a `PolyOps` trait containing separate implementations for `interpolate` and `evaluate`.

```