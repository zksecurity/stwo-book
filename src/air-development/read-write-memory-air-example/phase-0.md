# Phase 0: Setup

Before we start, let's set up our Rust project.

```bash
$ cargo new stwo-example
```

We need to specify the nightly Rust compiler to use Stwo.

```bash
$ echo -e "[toolchain]\nchannel = \"nightly-2025-01-02\"" > rust-toolchain.toml
```

Now let's edit the `Cargo.toml` file as follows:

```toml
[package]
name = "stwo-example"
version = "0.1.0"
edition = "2024"

[dependencies]
stwo-prover = { git = "https://github.com/starkware-libs/stwo.git", rev = "92984c060b49d0db05e021883755fac0a71a2fa7" }
num-traits = "0.2.17"
itertools = "0.12.0"
```

## Simplest AIR

Now that we have our setup, let's create the simplest possible AIR by adding the following code to `main.rs`.

This AIR creates a single column of zeros, and since Stwo doesn't allow us to create an AIR without constraints, we'll add a single constraint that requires all rows in the trace to be zero.

```rust
use num_traits::Zero;
use stwo_prover::{
    constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval, TraceLocationAllocator},
    core::{
        air::Component,
        backend::simd::{
            SimdBackend,
            column::BaseColumn,
            m31::{LOG_N_LANES, PackedM31},
        },
        channel::Blake2sChannel,
        fields::qm31::QM31,
        pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig},
        poly::circle::{CanonicCoset, CircleEvaluation, PolyOps},
        prover::{prove, verify},
        vcs::blake2_merkle::Blake2sMerkleChannel,
    },
};

type TestComponent = FrameworkComponent<TestEval>;

struct TestEval {
    log_size: u32,
}

impl FrameworkEval for TestEval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mask_item = eval.next_trace_mask();
        eval.add_constraint(mask_item);
        eval
    }
}

fn main() {
    // Setup
    let log_size = 4;
    let config = PcsConfig::default();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_size + 1 + config.fri_config.log_blowup_factor)
            .circle_domain()
            .half_coset,
    );
    let channel = &mut Blake2sChannel::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<_, Blake2sMerkleChannel>::new(config, &twiddles);

    // Prove
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(vec![]);
    tree_builder.commit(channel);

    let domain = CanonicCoset::new(log_size).circle_domain();
    let trace = CircleEvaluation::new(
        domain,
        BaseColumn::from_simd(vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)]),
    );
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(vec![trace]);
    tree_builder.commit(channel);

    let component = TestComponent::new(
        &mut TraceLocationAllocator::default(),
        TestEval { log_size },
        QM31::zero(),
    );
    let proof = prove(&[&component], channel, commitment_scheme).unwrap();

    // Verify
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);
    let sizes = component.trace_log_degree_bounds();

    commitment_scheme.commit(proof.commitments[0], &sizes[0], channel);
    commitment_scheme.commit(proof.commitments[1], &sizes[1], channel);

    verify(&[&component], channel, commitment_scheme, proof).unwrap();
}
```

This is a lot of code, but we'll go over what each part of the code does in detail in the next sections, so please keep reading!
