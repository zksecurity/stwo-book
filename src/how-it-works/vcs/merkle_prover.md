# Merkle Prover

This section explains the prover of the Merkle commitment scheme, focusing on how columns are committed to compute a Merkle root and how the Merkle tree layers are constructed, as well as how to generate Merkle proofs (decommitments).

## MerkleProver Structure

The `MerkleProver` struct represents a prover for a Merkle commitment scheme. It stores all layers of the Merkle tree, where each layer contains the hash values at that level.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/vcs/prover.rs 15:21}}
```

## The Commit Process

The core of the Merkle prover is the `commit` function, which takes as input a set of columns and outputs a `MerkleProver` containing all layers of the Merkle tree. The columns must be of length that is a power of 2.

Below is the complete implementation of the `commit` function:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/vcs/prover.rs 36:62}}
```

Let's walk through the function step by step:

1. **Sort Columns by Length:** Columns are sorted in descending order of length (columns with the most elements appear first). This ensures that the largest columns are committed first.

2. **Layer-by-Layer Commitment:**
     - For each layer (from largest to smallest), the function collects all columns of the current size and computes the hashes for that layer using the `commit_on_layer` function.
     - For the largest layer, the previous layer's hashes are empty, so the hash is computed directly from the column values.
     - For subsequent layers, the hash is computed from the previous layer's hashes and the current layer's column values.

3. **Reverse Layers:** After all layers are computed, the list of layers is reversed so that the root layer is at the beginning.

### Example: Commit Process

Suppose the input column data is as shown below:

<div style="text-align: center;">
    <figure id="fig-merkle-tree-data" style="display: inline-block;">
        <img src="../figures/merkle-tree-1.svg" width="500px" style="border-radius: 8px;" />
        <figcaption><span style="font-size: 0.9em">Figure 1: Example column data to commit using a Merkle tree</span></figcaption>
    </figure>
</div>

After sorting, the order is: $\textcolor{red}{\text{\textit{Column} 0}}$, $\textcolor{green}{\text{\textit{Column} 1}}$, $\textcolor{blue}{\text{\textit{Column} 2}}$ (from longest to shortest). We will now compute the hashes stored at each layer.

- **First Layer (Leaves):** The hashes are computed directly from the column values:
    $[h_{00}, h_{01}, h_{10}, h_{11}] = [H(\textcolor{red}{a}, \textcolor{green}{p}), H(\textcolor{red}{b}, \textcolor{green}{q}), H(\textcolor{red}{c}, \textcolor{green}{r}), H(\textcolor{red}{d}, \textcolor{green}{s})]$

- **Second Layer:** The next layer uses the previous hashes and the values from $\textcolor{blue}{\text{Column 2}}$:
    $[h_0, h_1] = [H(h_{00}, h_{01}, \textcolor{blue}{u}), H(h_{10}, h_{11}, \textcolor{blue}{v})]$

- **Root:** The root is computed as $root = H(h_0, h_1)$.

The resulting Merkle tree is illustrated below:

<div style="text-align: center;">
    <figure id="fig-merkle-tree" style="display: inline-block;">
        <img src="../figures/merkle-tree-2.svg" width="500px" style="border-radius: 8px;" />
        <figcaption><span style="font-size: 0.9em">Figure 2: Merkle tree structure after commitment</span></figcaption>
    </figure>
</div>


## The Decommit Process

The decommitment process enables the prover to generate a Merkle proof for a set of queried indices, allowing the verifier to check that specific elements are included in the committed Merkle tree.

The output is a `MerkleDecommitment` struct, which contains the hash and column values required for the verifier to reconstruct the Merkle root at the queried positions. Its implementation is shown below:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/verifier.rs 12:21}}
```

The `decommit` function implemented for the `MerkleProver` takes as input:
- `queries_per_log_size`: A map from log size to a vector of query indices for columns of that size.
- `columns`: The column data that was committed in the Merkle tree.

It returns:
- A vector of queried values, ordered as they are opened (from largest to smallest layer).
- A `MerkleDecommitment` containing the hash and column witnesses needed for verification.

Below is the complete implementation of the `decommit` function:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/vcs/prover.rs 78:152}}
```

Let's break down the function step by step:

1. **Sort Columns by Length:**
     - As in the `commit` function, columns are sorted in descending order of length.

2. **Layer-by-Layer Decommitment:**
     - For each layer (from largest to smallest):
         - Collect all columns of the current size (`layer_columns`) and the previous layer's hashes (`previous_layer_hashes`).
         - Retrieve the queries for the current layer (`layer_column_queries`) and the previous layer's queries (`prev_layer_queries`).
         - For each node index to be decommitted in this layer:
             - Add the necessary sibling hashes from the previous layer to the `hash_witness` in the decommitment.
             - If the node index is queried, fetch the corresponding column values and append them to `queried_values`.
             - If not queried, add the column values to the `column_witness` in the decommitment.
         - The set of node indices decommitted in this layer is propagated as queries to the next layer.


### Example: Decommit Process

For the column data in [Figure 1](#fig-merkle-tree-data), consider the query indices $[(2, [0]), (1, [1])]$, where the query indices are maps from log size to a vector of query indices for columns of that size. This corresponds to querying the following elements:
- The 0th element of columns of size $2^2=4$: $\textcolor{red}{a}$ from $\textcolor{red}{\text{\textit{Column} 0}}$ and $\textcolor{green}{p}$ from $\textcolor{green}{\text{\textit{Column} 1}}$.
- The 1st element of the column of size $2^1=2$: $\textcolor{blue}{v}$ from $\textcolor{blue}{\text{\textit{Column} 2}}$.

Because columns of equal length are committed together, the same indices are opened together in the decommitment. For example, for query $(2, [0])$, both $\textcolor{red}{a}$ and $\textcolor{green}{p}$ are opened together.

Below is a walkthrough of the main loop in the `decommit` function, showing the state of key variables for each `layer_log_size`:

- **layer_log_size = 2 (columns of size 4):**
    - `layer_columns`: $[\textcolor{red}{\text{\textit{Column} 0}}, \textcolor{green}{\text{\textit{Column} 1}}]$
    - `previous_layer_hashes`: `None` (first layer)
    - `prev_layer_queries`: empty
    - `layer_column_queries`: $[0]$
    - For `node_index = 0`:
        - `node_values`: $[\textcolor{red}{a}, \textcolor{green}{p}]$
        - Queried, so append to `queried_values`: $[\textcolor{red}{a}, \textcolor{green}{p}]$
        - Add `node_index = 0` to `layer_total_queries` (to propagate to next layer)
    - At this stage: `queried_values` = $[\textcolor{red}{a}, \textcolor{green}{p}]$, `decommitment` is empty.

- **layer_log_size = 1 (columns of size 2):**
    - `layer_columns`: $[\textcolor{blue}{\text{\textit{Column} 2}}]$
    - `previous_layer_hashes`: $[h_{00}, h_{01}, h_{10}, h_{11}]$
    - `prev_layer_queries`: $[0]$
    - `layer_column_queries`: $[1]$
    - For `node_index = 0`:
        - Add $h_{01}$ to `hash_witness` in decommitment.
        - `node_values`: $[\textcolor{blue}{u}]$
        - Not queried, so append to `column_witness` in decommitment.
        - Add `node_index = 0` to `layer_total_queries`.
    - At this stage: `queried_values` = $[\textcolor{red}{a}, \textcolor{green}{p}]$, `hash_witness` = $[h_{01}]$, `column_witness` = $[\textcolor{blue}{u}]$, `layer_total_queries` = $[0]$.
    - For `node_index = 1`:
        - Add $h_{10}, h_{11}$ to `hash_witness` in decommitment.
        - `node_values`: $[\textcolor{blue}{v}]$
        - Queried, so append to `queried_values`.
        - Add `node_index = 1` to `layer_total_queries`.
    - At this stage: `queried_values` = $[\textcolor{red}{a}, \textcolor{green}{p}, \textcolor{blue}{v}]$, `hash_witness` = $[h_{01}, h_{10}, h_{11}]$, `column_witness` = $[\textcolor{blue}{u}]$, `layer_total_queries` = $[0, 1]$ (to propagate to next layer).

- **layer_log_size = 0 (root):**
    - `layer_columns`: empty
    - `previous_layer_hashes`: $[h_{0}, h_{1}]$
    - `prev_layer_queries`: $[0, 1]$
    - `layer_column_queries`: empty
    - No values are added to `queried_values`, `hash_witness`, or `column_witness` in this layer.

**Final output:**
- `queried_values`: $[\textcolor{red}{a}, \textcolor{green}{p}, \textcolor{blue}{v}]$
- `decommitment`:
    - `hash_witness`: $[h_{01}, h_{10}, h_{11}]$
    - `column_witness`: $[\textcolor{blue}{u}]$

In the next section, we describe the verification process to verify the inclusion of the queried values using the Merkle decommitment.