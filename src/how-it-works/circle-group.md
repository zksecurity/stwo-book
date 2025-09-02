# Circle Group

As discussed in the previous section, Mersenne prime field $ \textsf{M31} $ lacks a smooth subgroup whose order is a large power of two. This property makes such fields unsuitable for instantiating STARK protocols. To address this, we consider extensions of $ \textsf{M31} $ that have smooth subgroups, which are suitable for performing FFTs and implementing the FRI protocol.

For a field extension $ F $ of $ \textsf{M31} $, we define the *circle curve* $ C(F) $ as the set of points $ (x, y) \in F^2 $ satisfying the relation:
$$ x^2 + y^2 = 1 $$

In Stwo implementation, a point on the circle is defined as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/circle.rs 13:16}}
```

The set $ C(F) $ forms a cyclic group under the operation defined by:
$$ (x,y) + (x', y') = (xx' - yy', xy' + x'y) $$ 

Here, the group is defined additively, which differs from the multiplicative notation used in the <a href="https://eprint.iacr.org/2024/278" target="_blank" rel="noopener noreferrer">Circle STARKs paper</a>. In this documentation, we adopt the additive notation for consistency with the implementation. The above group operation is implemented as: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/circle.rs 127:131}}
```

The identity element in this group is $ (1, 0) $, implemented as: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/circle.rs 19:24}}
```

Negation in the circle group corresponds to the conjugation map $ J $, defined by: 
$$ J(x, y) = (x, -y) $$ 
This is same as complex conjugation in complex numbers. In Stwo, the conjugate of a `CirclePoint` is computed as: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/circle.rs 92:97}}
```

The total number of points in the circle group $ C(F) $ is given by $ |F| + 1 $. Specifically, for $ C(\textsf{M31}) $, the number of points is $ P + 1 $, which, as discussed earlier, is a large power of two and can thus be used in STARK protocol instantiations. This result is proven in *Lemma 1* of the <a href="https://eprint.iacr.org/2024/278" target="_blank" rel="noopener noreferrer">Circle STARKs paper</a>.

In Stwo implementation, the generator $ g $ of the group $ C(\textsf{M31}) $ is defined as: 
$$ g = (2, 1268011823) $$ 
Subgroups of $ C(\textsf{M31}) $ of size $ 2^n $ can be generated using the following function: 
```rust,no_run,noplayground
    pub fn subgroup_gen(n: u32) -> CirclePoint<F> {
        assert!(n <= M31_CIRCLE_LOG_ORDER); // M31_CIRCLE_LOG_ORDER = 31
        let s = 1 << (M31_CIRCLE_LOG_ORDER - n);
        M31_CIRCLE_GEN.mul(s) // M31_CIRCLE_GEN = g = (2, 1268011823)
    }
```

To generate a subgroup $ \langle g_n \rangle $ of size $ 2^n $, the function computes $ 2^{31 - n} \cdot g $, i.e. it applies the group law to the generator $ g $ with itself $ 2^{31 - n} $ times, as shown below: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/circle.rs 71:82}}
```

Hence, the point $ 2^{31-n} \cdot g $ serves as a generator of a subgroup $ \langle g_n \rangle $ of order $ 2^n $.

## Circle Domain 

In a STARK protocol, the computation trace is interpolated as a low-degree polynomial over a domain using FFT. For Circle STARKs, this domain consists of points on the circle curve and is referred to as the *circle domain*. The circle domain $ D_n $ of size $ 2^n $ is constructed as the union of two disjoint cosets: 
$$ D_n = q + \langle g_{n-1} \rangle \cup -q + \langle g_{n-1} \rangle $$ 
Here, $ \langle g_{n-1} \rangle $ is a subgroup of size $ 2^{n-1} $, and $ q $ is the coset offset such that $ q \neq -q $. This union is also called the *twin-coset*. The second coset in the union can be viewed as the negation (or conjugation) of the first: 
$$ J(q + \langle g_{n-1} \rangle) = -q + \langle g_{n-1} \rangle $$ 
Therefore, it suffices to store only the half coset $ q + \langle g_{n-1} \rangle $, and generate the full domain via its conjugates. The circle domain is defined in Stwo as: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/circle/domain.rs 18:20}}
```

The following figure shows a circle domain of size 8. It is constructed from the half coset $ q + \langle g_2 \rangle $ of size 4 (shown as red points) and its negation $ -q + \langle g_2 \rangle $ (shown as blue points).

<div style="text-align: center;">
    <figure id="fig-circle-domain" style="display: inline-block;">
    <img src="./figures/circle-domain.svg" width="800px" style="border-radius: 8px;" />
        <figcaption><span style="font-size: 0.9em">Figure: Circle Domain of size 8</span></figcaption>
    </figure>
</div>

To iterate over all points in the circle domain, we can iterate over the half coset and its conjugates: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/circle/domain.rs 29:33}}
```

## Canonic Coset 

For a specific choice of offset $ q $, the twin-coset $ D_n $ becomes a coset of a larger subgroup. In particular, if $ q $ is a generator of a subgroup of order $ 2^{n+1} $, then: 
$$ D_n = q + \langle g_n \rangle = q + \langle g_{n-1} \rangle \cup -q + \langle g_{n-1} \rangle $$ 
This result is proven in *Proposition 1* of the <a href="https://eprint.iacr.org/2024/278" target="_blank" rel="noopener noreferrer">Circle STARKs paper</a>. Such domains are called *standard position coset*, or are referred to as *canonic cosets*. They are implemented as follows: 
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/circle/canonic.rs 23:25}}
```

Here, `CanonicCoset` represents the full coset $ q + \langle g_n \rangle $, while `CircleDomain` is represented with its half coset $ q + \langle g_{n-1} \rangle $. Thus to compute the `CircleDomain` from the `CanonicCoset`, first calculate the half coset $ q + \langle g_{n-1} \rangle $, which will be used to initialize the `CircleDomain` as shown below:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/circle/canonic.rs 46:48}}
```

The following figure shows a canonic coset of size 8. It is constructed from the coset $ \langle g_3 \rangle $ of size 8 followed by an offset by $ q $, where $ q $ is the generator of subgroup $ \langle g_4 \rangle $.

<div style="text-align: center;">
    <figure id="fig-canonic-coset" style="display: inline-block;">
    <img src="./figures/canonic-coset.svg" width="800px" style="border-radius: 8px;" />
        <figcaption><span style="font-size: 0.9em">Figure: Canonic Coset of size 8</span></figcaption>
    </figure>
</div>

We can verify whether a given `CircleDomain` is *canonic* by checking the step size of the half coset against the initial coset offset. In the `CircleDomain` implementation, only the *half coset* $ q + \langle g_{n-1} \rangle $ is explicitly stored. If `CircleDomain` is canonic, $ q $ must be a generator of the subgroup $ \langle g_{n+1} \rangle $, which has order $ 2^{n+1} $ i.e. $ q = 2^{31 - (n+1)} \cdot g $. Recall that the generator of the subgroup $ \langle g_{n-1} \rangle $ is $ 2^{31 - (n-1)} \cdot g $.

Thus, the step size between consecutive elements in the half coset is $ 2^{31 - (n-1)} \cdot g $, and the initial point is $ q = 2^{31 - (n+1)} \cdot g $. Therefore, the ratio between the step size and the initial coset offset is:
$$
\frac{2^{31 - (n-1)}}{2^{31 - (n+1)}} = 2^2 = 4
$$
This means that in a canonic coset, the step size is exactly four times the initial coset offset. This condition is used to check whether a `CircleDomain` is canonic, as shown below:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/circle/domain.rs 71:73}}
```

In the next section, we will dive into polynomials defined over the circle.
