# Polynomial Commitment Scheme Verifier


In this section, we describe the implementation of the polynomial commitment scheme verifier.

## Commitment Scheme Verifier


The `CommitmentSchemeVerifier` struct manages the verification process for the polynomial commitment scheme. It maintains a collection of [Merkle verifiers](../vcs/merkle_verifier.md) (one for each commitment tree) and the protocol configuration.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/pcs/verifier.rs 21:24}}
```


We will now see some key functions defined for the `CommitmentSchemeVerifier` struct.

### Read Commitments

The `commit` function reads a Merkle root from the prover and initializes a `MerkleVerifier` for the committed columns. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/pcs/verifier.rs 42:55}}
```

### Verify


The `verify_values` function is the core of the verification protocol. It checks that the prover's openings at the sampled points are consistent with the commitments and that the committed polynomials are of low degree via the FRI protocol. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/pcs/verifier.rs 57:126}}
```



Here is a detailed breakdown:

1. **Mix Sampled Values into the Fiat-Shamir Channel**:
   - The verifier mixes the `sampled_values` (openings at the queried points) into the Fiat-Shamir channel, ensuring that all subsequent randomness is bound to these values.

2. **Draw Random Coefficient**:
   - The verifier draws a `random_coeff` from the channel, which is used to combine the quotient polynomials in the FRI protocol.

3. **Determine Degree Bounds**:
   - The verifier computes the degree `bounds` for each column, based on the log sizes and the protocol's blowup factor. These bounds are used to configure the FRI verifier.

4. **FRI Commitment Phase**:
   - The verifier initializes the `fri_verifier` with the FRI protocol configuration, the FRI proof from the prover, and the degree bounds.

5. **Verify Proof of Work**:
   - The verifier checks the `proof_of_work` value provided by the prover using the `pow_bits` in the PCS config.

6. **Sample FRI Query Positions**:
   - The verifier uses the channel to generate random `query_positions_per_log_size` for the FRI protocol.

7. **Verify Merkle Decommitments**:
   - For each commitment tree, the verifier checks that the Merkle decommitments at the queried positions are valid and that the opened values match the commitments.

8. **Prepare FRI Query Answers**:
   - The verifier assembles the answers to the FRI queries by matching the sampled points and values, and prepares them for the FRI verifier.

9. **FRI Decommitment Phase**:
   - The verifier provides the FRI query answers to the FRI verifier, which checks that the quotient polynomials are of low degree.

10. **Return Verification Result**:
   - If all checks pass, the function returns `Ok(())`. If any check fails (e.g., Merkle decommitment, proof of work, or FRI check), it returns an appropriate error.
