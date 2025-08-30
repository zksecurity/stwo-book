# Secure Evaluations
Similar to `CircleEvaluation`, `SecureEvaluation` is a [point-value representation](./evals-and-poly.md#point-value-representation) of a polynomial whose evaluations over the `CircleDomain` are from the `SecureField` (an alias for `QM31`). This is implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/poly/circle/secure_poly.rs 60:69}}
```

As discussed in the [previous subsection](./columns.md#secure-field-columns), each `SecureField` element is represented by four base field elements and stored in four parallel columns. Thus the evaluations are represented as `SecureColumnByCoords`, as shown above. 

Similar to `CircleEvaluation`, we can interpolate a `SecureCirclePoly` with coefficients from the `SecureField` as shown below:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/poly/circle/secure_poly.rs 111:122}}
```


# Secure Circle Polynomials

Similar to `CirclePoly`, `SecureCirclePoly` is a [coefficient representation](./evals-and-poly.md#coefficient-representation) of a polynomial whose coefficients are from the `SecureField`. As discussed in the [earlier section](./evals-and-poly.md#eq-circle-poly), we can define a circle polynomial as follows:
\\[
p(x, y) = \sum_{j=0}^{2^n -1} v_j \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdot \pi^2(x)^{j_3} \cdots \pi^{n-2}(x)^{j\_{n-1}}
\\]
Here, \\( v_j \\) are the coefficients from `SecureField` (i.e. \\( \mathsf{QM31} \\)). We can represent the coefficient \\( v_j \\) using four elements from the `BaseField` (i.e. \\( \mathsf{M31} \\)) as follows:
\\[ v_j = (a_j + i \cdot b_j) + (c_j + i \cdot d_j) \cdot u \\] 

where \\( a_j, b_j, c_j, d_j \in \mathsf{M31} \\). Substituting the above representation in the equation of \\( p(x, y) \\) we get:
\\[
p(x, y) = \sum_{j=0}^{2^n -1} \Big(a_j + i \cdot b_j + u \cdot c_j + i \cdot u \cdot d_j \Big) \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdot \pi^2(x)^{j_3} \cdots \pi^{n-2}(x)^{j\_{n-1}}
\\]


\\[
p(x, y) = \sum_{j=0}^{2^n -1} a_j \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdots \pi^{n-2}(x)^{j\_{n-1}} \\ + \\ i \cdot \sum_{j=0}^{2^n -1} b_j \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdots \pi^{n-2}(x)^{j\_{n-1}} \\ + \\
\\]

\\[
\\ \\ u \cdot \sum_{j=0}^{2^n -1} c_j \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdots \pi^{n-2}(x)^{j\_{n-1}} \\ + \\ iu \cdot \sum_{j=0}^{2^n -1} d_j \cdot y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdots \pi^{n-2}(x)^{j\_{n-1}}
\\]

Thus we can represent a `SecureCirclePoly` using four `CirclePoly`: \\( p_a(x, y), p_b(x, y), p_c(x, y) \\) and \\( p_d(x, y) \\) as follows:
\\[ 
p(x, y) = p_a(x, y) + i \cdot p_b(x, y) + u \cdot p_c(x, y) + iu \cdot p_d(x, y)
\\]

where \\( p_a(x,y) \\) is a `CirclePoly` with coefficients \\( a_j \in \mathsf{M31} \\), similarly for \\( p_b(x, y), p_c(x, y) \\) and \\( p_d(x, y) \\). This is implemented as follows:

<!-- TODO: Add a figure or example to explain this implementation. -->
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/poly/circle/secure_poly.rs 14:14}}
```

Here, `SECURE_EXTENSION_DEGREE` is the degree of extension of the `SecureField`, which is 4.

Similar to `CirclePoly`, we can evaluate the `SecureCirclePoly` at points on the given `CircleDomain` which outputs the `SecureEvaluation`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/poly/circle/secure_poly.rs 37:45}}
```

In the next section, we will see how the `interpolate` and `evaluate` functions convert between the two polynomial representations using Circle FFT. As you may have noticed, the twiddles are precomputed for efficiency, we will also explore this in the next section on Circle FFT.