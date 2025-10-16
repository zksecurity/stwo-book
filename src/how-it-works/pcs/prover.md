# Polynomial Commitment Scheme Prover


In this section, we will see the implementation of the commitment scheme prover. We will start by looking at the building blocks.

## Commitment Tree Prover


The `CommitmentTreeProver` struct represents the data for a single Merkle tree commitment. As we have seen in the [Merkle tree section](../vcs/merkle_prover.md#merkle-prover), we can commit to multiple polynomials of different degrees in the same Merkle tree. It is implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 198:202}}
```

Here, `pub type ColumnVec<T> = Vec<T>`. It contains the following fields:
- `polynomials`: The set of polynomials committed in a single Merkle tree.
- `evaluations`: The evaluations of these polynomials over their respective domains.
- `commitment`: The `MerkleProver` struct as described in the [Merkle tree section](../vcs/merkle_prover.md#merkleprover-structure).

It is initialized as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 205:224}}
```

It proceeds as follows. First, given the `polynomials`, we evaluate them on the evaluation domain using circle FFT to compute `evaluations`. Then we commit to those evaluations using the `MerkleProver` struct. Finally, we create and output the `CommitmentTreeProver` struct.

## Commitment Scheme Prover


The `CommitmentSchemeProver` struct is the key struct which maintains a vector of commitment trees. It implements functionalities to open the committed polynomials, compute quotients, and then apply the FRI protocol. It contains the following fields:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 27:31}}
```

It contains the following fields:
- `tree`: This contains a vector of commitment trees. Here, `pub struct TreeVec<T>(pub Vec<T>)`.
- `config`: This is the `PcsConfig`, which contains the `fri_config` and `pow_bits`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/pcs/mod.rs 29:32}}
```
The security of the polynomial commitment scheme is computed as:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/pcs/mod.rs 34:36}}
```
- `twiddles`: This contains [precomputed twiddle factors](../circle-fft/twiddles.md#twiddle-tree).


Now we will see some key functions defined on the `CommitmentSchemeProver` struct.

### Commit


The `commit` function, given a batch of polynomials, computes the `CommitmentTreeProver` struct which commits to the input polynomials and then appends the `tree` struct to the vector of stored `trees`.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 42:51}}
```

### Trace

The `trace` function returns a `Trace` struct containing all polynomials and their evaluations corresponding to all the commitment trees. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 79:83}}
```

### Commitment Tree Builder


The `tree_builder` function outputs the `TreeBuilder` struct.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 53:59}}
```


The `TreeBuilder` struct is a helper for aggregating polynomials and evaluations before committing them in a Merkle tree. It allows the prover to collect columns (polynomials) and then commit them together as a batch. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 159:163}}
```


### Prove


The `prove_values` function is central to the protocol, handling the opening of committed polynomials at specified sample points and integrating with the FRI protocol for low-degree testing. It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/pcs/mod.rs 85:156}}
```


Here is a detailed breakdown:

1. **Evaluate Polynomials at Sample Points**:
   - For each committed polynomial and each sample point (including out-of-domain points and mask points which contain constraint offsets), the function evaluates the polynomials and collects the results in `samples`.
   - The `sampled_values` are mixed into the channel, ensuring they are bound to the proof and used for subsequent randomness generation.

2. **Compute FRI Quotients**:
   - The function computes FRI quotient polynomials using `compute_fri_quotients` to open the committed polynomials at sampled points in `samples`. This follows the same quotienting process as described in the [overview section](./overview.md#polynomial-commitment-scheme).

3. **FRI Commitment Phase**:
   - The FRI protocol is run on the quotient polynomials, committing to their evaluations in Merkle trees and initializing the `fri_prover`. For more details, refer to the [FRI prover section](../circle-fri/fri_prover.md).

4. **Proof of Work**:
   - A proof-of-work step is performed, with the result mixed into the channel.

6. **FRI Decommitment Phase**:
   - The function generates random query positions using the channel and decommits the FRI layers at those positions, providing Merkle decommitments for all queried values. For more details, refer to the [FRI prover section](../circle-fri/fri_prover.md).

7. **Decommitment of Committed Trees**:
   - For each commitment tree, the function decommits the Merkle tree at the FRI query positions, providing the queried values and authentication paths.

8. **Return Proof Object**:
   - The function returns a `CommitmentSchemeProof` object containing:
     - Merkle roots of all commitments
     - Sampled values at all sample points
     - Merkle decommitments for all queries
     - Queried values
     - Proof-of-work result
     - FRI proof
     - Protocol configuration



We will now look into the proof verifier implementation.
