use stwo_prover::core::{
    backend::{
        simd::{
            column::BaseColumn,
            m31::{LOG_N_LANES, N_LANES},
            SimdBackend,
        },
        Column,
    },
    channel::Blake2sChannel,
    fields::m31::M31,
    pcs::{CommitmentSchemeProver, PcsConfig},
    poly::{
        circle::{CanonicCoset, CircleEvaluation, PolyOps},
        BitReversedOrder,
    },
    vcs::blake2_merkle::Blake2sMerkleChannel,
    ColumnVec,
};

const CONSTRAINT_EVAL_BLOWUP_FACTOR: u32 = 1;

fn main() {
    let num_rows = N_LANES;
    let log_num_rows = LOG_N_LANES;

    // Create the table
    let mut col_1 = BaseColumn::zeros(num_rows);
    col_1.set(0, M31::from(1));
    col_1.set(1, M31::from(7));

    let mut col_2 = BaseColumn::zeros(num_rows);
    col_2.set(0, M31::from(5));
    col_2.set(1, M31::from(11));

    // Convert table to trace polynomials
    let domain = CanonicCoset::new(log_num_rows).circle_domain();
    let trace: ColumnVec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>> = vec![col_1, col_2]
        .into_iter()
        .map(|col| CircleEvaluation::new(domain, col))
        .collect();

    // Config for FRI and PoW
    let config = PcsConfig::default();

    // Precompute twiddles for evaluating and interpolating the trace
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(
            log_num_rows + CONSTRAINT_EVAL_BLOWUP_FACTOR + config.fri_config.log_blowup_factor,
        )
        .circle_domain()
        .half_coset,
    );

    // Create the channel and commitment scheme
    let channel = &mut Blake2sChannel::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(config, &twiddles);

    // Commit to the preprocessed trace
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(vec![]);
    tree_builder.commit(channel);

    // Commit to the original trace
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(trace);
    tree_builder.commit(channel);
}
