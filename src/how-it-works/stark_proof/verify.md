# STARK Verifier

This section provides an overview of the `verify` function, the key function that verifies the STARK proof. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/verifier.rs 14:74}}
```


Let us go through the function in detail.

## Input and Output


The `verify` function is the entry point for verifying a STARK proof. It takes as input:
- `components`: A list of AIR components. For more details, refer to the [Components](../air/components.md) section.
- `channel`: A Fiat-Shamir channel for non-interactive randomness.
- `commitment_scheme`: A `CommitmentSchemeVerifier` for verifying Merkle commitments and FRI proofs. For more details, refer to the [PCS Verifier section](../pcs/verifier.md).
- `proof`: The `StarkProof` object to be verified.


It returns `Ok(())` if the proof is valid, or a `VerificationError` if any check fails.

## Step-by-Step Breakdown


1. **Determine Preprocessed Columns**
   - The function determines the number of preprocessed columns, `n_preprocessed_columns`, from the `commitment_scheme`, which is used to initialize the `Components` struct.


2. **Initialize Components**
   - The `Components` structure is created, encapsulating all AIR components and the number of preprocessed columns.


3. **Read Composition Polynomial Commitment**
   - The verifier reads the Merkle root of the composition polynomial from the `proof` and registers it with the commitment scheme verifier, along with the degree bounds for each coordinate polynomial.


4. **Out-of-Domain Sampling (OODS)**
   - An `oods_point` is drawn randomly from the channel. This point is used to bind the prover to a unique low-degree polynomial and prevent ambiguity in the list decoding regime.


5. **Determine Sample Points**
   - The function computes all `sample_points` required to verify constraints at the OODS point, using the `mask_points` function. This includes all necessary offsets for each constraint and the OODS points for the composition polynomial.


6. **Sanity Check: Composition Polynomial Evaluation**
   - The function checks that the composition polynomial evaluated at the OODS point (as provided in the proof) matches the value reconstructed from the sampled trace values. If not, it returns an `OodsNotMatching` error.


7. **Invoke Commitment Scheme Verifier**
   - The function calls `verify_values` on the commitment scheme verifier, passing the `sample_points`, the `proof`, and the `channel`. This step checks all Merkle decommitments, FRI low-degree proofs, and protocol soundness.


8. **Return Verification Result**
   - If all checks pass, the function returns `Ok(())`. If any check fails (e.g., Merkle decommitment, FRI check, or OODS mismatch), it returns an appropriate `VerificationError`.
