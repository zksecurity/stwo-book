# FRI Prover


In this section, we examine the FRI prover implementation, beginning with the FRI protocol configuration.

## FRI Protocol Configuration


We configure the FRI protocol using the following parameters:
- Log of blowup factor $\beta$
- Log of last layer degree bound (determines the number of rounds $r$ in the FRI protocol)
- Number of queries $s$ made by the verifier in the query phase

It is implemented as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/fri.rs 29:34}}
```

We calculate the security bits of our protocol as follows:
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/fri.rs 68:70}}
```
This is as we discussed in the [Security Analysis section](./overview.md#security-analysis).

## Proving

Let us look into how the FRI prover struct is implemented.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 77:82}}
```

Here, `FriOps` is a trait which implements functionality for the commit phase of FRI, such as folding the evaluations, and `MerkleOps` is the trait used in the [Merkle commitment scheme](../vcs/hash_functions.md#merkleops-trait). The generic `B` refers to a specific backend, for example either `CpuBackend` or `SimdBackend`, which implements the `FriOps` and `MerkleOps` traits.


We described [FRI](./overview.md#protocol) as an interactive protocol between the prover and the verifier. To make the protocol non-interactive, we use the [Fiat-Shamir transform](https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic), where both the prover and verifier use a channel to hash the transcript and generate random challenges. These functionalities are defined by the `MerkleChannel` trait. In the non-interactive protocol, oracles to functions are replaced by Merkle commitments to their evaluations, and queries to the oracle by the verifier are replaced by Merkle decommitments, which the prover appends to the channel.


The `FRIProver` struct is composed of several layers. Each layer contains a Merkle tree that commits to the evaluations of a polynomial for that layer. The main components are:

**• `config`**: The `FriConfig` discussed in the previous section, which holds protocol parameters.

**• `first_layer`**: The first layer of the FRI protocol, containing the commitment to the initial set of columns. 
  
   ```rust,no_run,noplayground
   {{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 278:281}}
   ```
   For example, the `columns` are the array of evaluations $[h_{0}, h_{1}, h_{2}]$, and `merkle_tree` commits to $h_{0} \in F^{H_0}$, $h_{1} \in F^{H_1}$, and $h_{2} \in F^{H_2}$ using a single Merkle tree.

**• `inner_layers`**: The inner layers of FRI, each representing a folding round and its corresponding Merkle commitment.
  
   ```rust,no_run,noplayground
   {{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 363:366}}
   ```
   In our example, there are two FRI inner layers: the first contains evaluations of $g_0$ over the "line" domain $F^{I_0}$ with a Merkle commitment to $g_0$, and the second contains evaluations of $g_1$ over $F^{I_1}$ with its Merkle commitment.

**• `last_layer_poly`**: The last layer polynomial, which the prover sends in clear to the verifier.
  
   ```rust,no_run,noplayground
   {{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/poly/line.rs 108:116}}
   ```
   For our example, this is the polynomial $g_2$ in coefficient representation.

### Commitment

The `commit` function corresponds to the commitment phase of our protocol and outputs the `FriProver` struct. This function handles multiple mixed-degree polynomials, each evaluated over domains of different sizes. We will now give a high-level overview of the function as it is implemented in Stwo.

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 102:129}}
```
It takes the following inputs:
- `channel`: The Merkle channel used for the Fiat-Shamir transform to generate random challenges and maintain the transcript.
- `config`: The `FriConfig` containing protocol parameters.
- `columns`: The array of evaluations of the functions. For our example, this will contain $[h_0, h_1, h_2]$ over their respective domains $[H_0, H_1, H_2]$.
- `twiddles`: The [precomputed twiddle](../circle-fft/twiddles.md#twiddle-tree) values needed for folding.


The commitment phase consists of the following steps, corresponding to the protocol rounds described in the [overview](./overview.md#protocol):

1. **First Layer Commitment** (`commit_first_layer`): 
   - Takes the input functions $[h_0, h_1, h_2]$ and creates a Merkle commitment to all of them using a single Merkle tree.
   - Commits to the root of the Merkle tree by appending it to the channel as part of the transcript.
   - Returns the `FriFirstLayerProver` containing the columns and their Merkle commitment.

2. **Inner Layers Commitment** (`commit_inner_layers`):
    - Performs the folding rounds as described in the protocol.
    - In each round $i$:
       - Decomposes the previous round "line" polynomial $g_{i-1}$ into $g_{i-1, 0}$ and $g_{i-1, 1}$.
       - Decomposes the current round "circle" polynomial $h_i$ into $h_{i, 0}$ and $h_{i, 1}$.
       - Receives random challenge $\lambda_i$ from the channel.
       - Folds the decomposed functions to compute $g_i$ over domain $I_i$.
       - Creates Merkle commitment to $g_i$ and adds the root of the Merkle tree to the channel.
    - For our example with $r=2$, this creates two inner layers containing $g_0$ and $g_1$.
    - Returns the following two objects:
       - Two `FriInnerLayerProver` corresponding to $g_0$ and $g_1$.
       - Final `last_layer_evaluation`, i.e., evaluations of $g_2$ over the domain $I_2$.

3. **Last Layer Commitment** (`commit_last_layer`):
   - Takes the final evaluation $g_r$ (which will be sent to the verifier in clear).
   - Interpolates it to coefficient form and appends the coefficients into the channel as protocol transcript.
   - For our example, this converts $g_2 \in F^{I_2}$ to polynomial coefficient representation.
   - Returns the `last_layer_poly`.


The function then constructs and returns the complete `FriProver` struct containing all layers, which will be used later for decommitment during the query phase.

### Decommitment


Now we will look at the decommit function. It is implemented as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 232:240}}
```

It takes the following input:
1. `self`: The `FriProver` containing the Merkle tree commitments to all the FRI layers.
2. `channel`: The Fiat-Shamir channel used to hash the transcript and generate the random query points.

Let us walk through the function step by step.
1.  **Setup Query Generation**: Use the Fiat-Shamir channel to generate `n_queries` random positions on the maximum domain.
2. **Map Query Positions by Domain Size**: The function `get_query_positions_by_log_size` takes `queries` and `column_log_sizes` as input and maps each domain size to its respective query position in the column.
3. **Generate Proof**: The function `decommit_on_queries` generates the proof `FriProof` using the queries. The struct `FriProof` contains the Merkle decommitments for each layer with respect to the query positions.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/fri.rs 422:426}}
```


For our example, the components of `FriProof` will be as follows:
- `first_layer`: The decommitments to query positions for $h_0$, $h_1$, and $h_2$.
- `inner_layers`: There will be two inner layer proofs, i.e., one for the decommitments of $g_0$ and another for decommitments of $g_1$.
- `last_layer_poly`: This will be the $g_2$ polynomial represented in coefficient form.


4. Return the following objects:
   - `proof`: The `FriProof` struct with all layer decommitments.
   - `query_positions_by_log_size`: The query mapping from domain log sizes to their respective query positions.



Now let us look at the key function `decommit_on_queries` in detail.
```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/fri.rs 245:272}}
```

The function `decommit_on_queries` generates the `FriProof` struct by decommitting all layers. Suppose there is a single query corresponding to point $P_0 = (x_0, y_0) \in H_0$ and let $P_i = \pi^i(x_i, y_i) \in H_i$.
1. **Decommit First Layer**: This provides Merkle tree decommitments for queried positions with respect to the first layer. This provides evaluations $h_0(P_0), h_0(-P_0)$, $h_1(P_1), h_1(-P_1)$, and $h_2(P_2), h_2(-P_2)$ along with their Merkle decommitments in the Merkle tree containing the first layer.
2. **Process Inner Layers with Folding**: We process the decommitment layer by layer. For our example, this proceeds as follows:
   - For the first inner layer: Provide the evaluation $g_0(x_0), g_0(-x_0)$ along with their Merkle decommitments.
   - For the second inner layer: Provide the evaluation $g_1(x_1), g_1(-x_1)$ along with their Merkle decommitments.
3. **Assemble Final Proof**: Combines all layer decommitments with the last layer polynomial $g_2$ into `FriProof`.