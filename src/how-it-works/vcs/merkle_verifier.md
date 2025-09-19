# Merkle Verifier

This section covers the verification component of the Merkle commitment scheme. The following struct implements the verifier of the Merkle commitment scheme.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/verifier.rs 31:35}}
```

The struct elements are defined as follows:
* `root`: root hash of the Merkle tree committing to column data
* `column_log_sizes`: a vector containing log size values of all the columns
* `n_columns_per_log_size`: a map that associates each column log size with the number of columns of that size


## Verifying the decommitment

The `verify` function is the main function defined for the `MerkleVerifier`. It takes the following input:
- `queries_per_log_size`: A map from log size to a vector of query indices for columns of that size.
- `queried_values`: The queried column values, which is one of the outputs of the `decommit` function.
- `decommitment`: `MerkleDecommitment` containing the `hash_witness` and `column_witness` required to check inclusion of the `queried_values` in the Merkle tree. This is also one of the outputs of the `decommit` function.

Below is the complete implementation of the `verify` function:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/verifier.rs 72:190}}
```

Let's break down the function step by step:

1. **Initialize Variables:**
   - Convert `queried_values` into an iterator for consumption during verification.
   - Create iterators for `hash_witness` and `column_witness` from the `decommitment`.
   - Initialize `last_layer_hashes` to track computed hashes from the previous layer.

2. **Layer-by-Layer Verification:**
   - For each layer (from largest to smallest):
     - Get the number of columns in the current layer (`n_columns_in_layer`) from the map `n_columns_per_log_size`.
     - Prepare iterators for previous layer queries (`prev_layer_queries`), previous layer hashes (`prev_layer_hashes`) and current layer column queries (`layer_column_queries`).
     - For each node index:
       - **Reconstruct `node_hashes`:** Use computed hashes from the `prev_layer_hashes` or read missing sibling hashes from `hash_witness`.
       - **Get Node Values:** If the node is queried, read column values from `queried_values`; otherwise, read from `column_witness`.
       - **Compute Hash:** Use the hash function to compute the hash of the current node from its children hashes and column values.
       - Store the computed hash for propagation to the next layer.

3. **Final Verification:**
   - Check that all witnesses and queried values have been fully consumed (no excess data).
   - Verify that the computed root matches the expected root stored in the verifier.
   - Return `Ok(())` if verification succeeds, or an appropriate error otherwise.

### Example: Verify the decommitment

The same example from the [decommit process](./merkle_prover.md#example-decommit-process) is used to verify the output of the `decommit` function. The input to the `verify` function is as follows:
- `queries_per_log_size`: $[(2, [0]), (1, [1])]$
- `queried_values`: $[\textcolor{red}{a}, \textcolor{green}{p}, \textcolor{blue}{v}]$
- `decommitment`:
    - `hash_witness`: $[h_{01}, h_{10}, h_{11}]$
    - `column_witness`: $[\textcolor{blue}{u}]$

Below is a walkthrough of the main loop in the `verify` function, showing how the verifier reconstructs the Merkle root:

- **layer_log_size = 2 (columns of size 4):**
    - `n_columns_in_layer`: 2 (for $\textcolor{red}{\text{\textit{Column} 0}}$ and $\textcolor{green}{\text{\textit{Column} 1}}$)
    - `last_layer_hashes`: `None` (first layer)
    - `prev_layer_queries`: empty
    - `layer_column_queries`: $[0]$
    - For `node_index = 0`:
        - `node_hashes`: `None` (no previous layer)
        - `node_values`: Read from `queried_values` → $[\textcolor{red}{a}, \textcolor{green}{p}]$
        - Compute hash: $h_{00} = H(\textcolor{red}{a}, \textcolor{green}{p})$
        - Add to `layer_total_queries`: $(0, h_{00})$
    - At this stage: `last_layer_hashes` = $[(0, h_{00})]$

- **layer_log_size = 1 (columns of size 2):**
    - `n_columns_in_layer`: 1 (for $\textcolor{blue}{\text{\textit{Column} 2}}$)
    - `last_layer_hashes`: $[(0, h_{00})]$
    - `prev_layer_queries`: $[0]$
    - `layer_column_queries`: $[1]$
    - For `node_index = 0`:
        - `node_hashes`: Left child is $h_{00}$ (computed from `last_layer_hashes`), right child $h_{01}$ read from `hash_witness`
        - `node_values`: Read from `column_witness` → $[\textcolor{blue}{u}]$
        - Compute hash: $h_0 = H(h_{00}, h_{01}, \textcolor{blue}{u})$
        - Add to `layer_total_queries`: $(0, h_0)$
    - For `node_index = 1`:
        - `node_hashes`: Both children $h_{10}, h_{11}$ read from `hash_witness`
        - `node_values`: Read from `queried_values` → $[\textcolor{blue}{v}]$
        - Compute hash: $h_1 = H(h_{10}, h_{11}, \textcolor{blue}{v})$
        - Add to `layer_total_queries`: $(1, h_1)$
    - At this stage: `last_layer_hashes` = $[(0, h_0), (1, h_1)]$

- **layer_log_size = 0 (root):**
    - `n_columns_in_layer`: 0 (no columns of size 1)
    - `last_layer_hashes`: $[(0, h_0), (1, h_1)]$
    - `prev_layer_queries`: $[0, 1]$
    - `layer_column_queries`: empty
    - For `node_index = 0`:
        - `node_hashes`: Left child is $h_0$, right child is $h_1$ (both computed from `last_layer_hashes`)
        - `node_values`: empty (no columns)
        - Compute hash: $root = H(h_0, h_1)$
        - Add to `layer_total_queries`: $(0, root)$
    - At this stage: `last_layer_hashes` = $[(0, root)]$

**Final verification:**
- Check that all iterators are exhausted: `hash_witness`, `queried_values`, and `column_witness` should all be empty.
- Compare the computed `root` with the expected root stored in the `MerkleVerifier`.
- If they match, return `Ok(())`; otherwise, return `Err(MerkleVerificationError::RootMismatch)`.