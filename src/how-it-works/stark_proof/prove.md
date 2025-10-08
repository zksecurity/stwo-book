# STARK Prover


This section provides an overview of the `prove` function, the key function in the STARK proof generation process. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/mod.rs 27:84}}
```


Let us go through the function in detail.

## Input and Output


It takes the following as input:
- `components`: A list of AIR components. For more details, refer to the [Components](../air/components.md) and [Prover Components](../air/prover_components.md) sections.
- `channel`: A Fiat-Shamir channel for non-interactive randomness.
- `commitment_scheme`: A `CommitmentSchemeProver` for committing to trace and composition polynomials. For more details, refer to the [PCS Prover section](../pcs/prover.md).


It outputs a `StarkProof` object if successful, or a `ProvingError` if any constraint is not satisfied. The `StarkProof` object is a wrapper around `CommitmentSchemeProof`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/proof.rs 16:16}}
```

## Step-by-Step Breakdown


1. **Determine Preprocessed Columns**
   - The function determines the number of preprocessed columns, `n_preprocessed_columns`, from the `commitment_scheme`, which is used to initialize the `ComponentProvers` structure.


2. **Collect Trace Data**
   - The `trace`, containing all columns (execution, interaction, preprocessed), is retrieved from the `commitment_scheme`. This includes both coefficient and evaluation forms for each column.


3. **Composition Polynomial Construction**
   - A `random_coeff` is drawn from the channel.
   - The `composition_poly` is computed as a random linear combination of all constraint quotient polynomials, using powers of the random coefficient. For more details, refer to the [Prover Components](../air/prover_components.md) section.


4. **Commit to the Composition Polynomial**
   - The `composition_poly` is split into coordinate polynomials and committed to using a Merkle tree.


5. **Out-of-Domain Sampling (OODS)**
   - An `oods_point` is drawn randomly from the channel. This point is used to bind the prover to a unique low-degree polynomial, preventing ambiguity in the list decoding regime. For more details, refer to the [Out-of-Domain Sampling](../pcs/overview.md#out-of-domain-sampling) section.


6. **Determine Sample Points**
   - The function computes all `sample_points` required to verify constraints at the OODS point, using the `mask_points` function. This includes all necessary offsets for each constraint and the OODS points for the composition polynomial.


7. **Openings and Proof Generation**
   - The `commitment_scheme` is asked to open all committed polynomials at the sampled points, producing the required evaluations and Merkle authentication paths. This is handled by the `prove_values` function, which also integrates the FRI protocol for low-degree testing. For more details, refer to the [PCS Prover](../pcs/prover.md#prove) section.


8. **Sanity Check**
   - The function checks that the composition polynomial evaluated at the OODS point matches the value reconstructed from the sampled trace values. If not, it returns a `ConstraintsNotSatisfied` error.


9. **Return Proof**
   - If all checks pass, the function returns a `StarkProof` object containing the full proof transcript, including all commitments, openings, and FRI proof.
