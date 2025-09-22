
# Hash Functions

This section describes the traits and implementations of hash functions used in the Merkle commitment scheme. Stwo supports two hash functions: **BLAKE2s-256** and **Poseidon252**. Here, we focus on the implementation for BLAKE2s-256; Poseidon252 is implemented similarly (see [Poseidon reference](https://docs.starknet.io/learn/protocol/cryptography#poseidon-hash)).


## MerkleHasher Trait

The `MerkleHasher` trait defines the interface for hash functions used in Merkle trees. Its main function, `hash_node`, computes the hash of a node from its children and (optionally) column values:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/merkle_hasher.rs 12:19}}
```

### Implementation for BLAKE2s-256 

The `MerkleHasher` implementation for BLAKE2s-256 uses a wrapper struct `Blake2sHash`:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/blake2_hash.rs 11:11}}
```

The trait implementation is provided by `Blake2sMerkleHasher`:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/core/vcs/blake2_merkle.rs 10:31}}
```

In this Merkle tree implementation, node hashes are computed using both the children hashes and the column values. This differs from standard Merkle trees, where node hashes typically depend only on the children. More details are discussed in the next sections.

## MerkleOps Trait

The `MerkleOps` trait defines Merkle tree operations for a commitment scheme, parameterized by a `MerkleHasher`. Its main function, `commit_on_layer`, takes the previous layer's hashes and the current layer's column values to generate the hashes for the next layer:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/vcs/ops.rs 8:29}}
```

### Implementation for BLAKE2s-256

The `MerkleOps<Blake2sMerkleHasher>` trait implementation for the `CpuBackend` is as follows:

```rust,no_run,noplayground
{{#webinclude https://raw.githubusercontent.com/starkware-libs/stwo/0790eba46b8af5697083d84fb75bd34b08a0b31f/crates/stwo/src/prover/backend/cpu/blake2s.rs 10:25}}
```

In the next section, we will use these hash function implementations to describe the prover of the Merkle commitment scheme.
