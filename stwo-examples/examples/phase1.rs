use itertools::Itertools;
use num_traits::{One, Zero};

use stwo_prover::{
    constraint_framework::{
        logup::{LogupTraceGenerator, LookupElements},
        EvalAtRow, FrameworkComponent, FrameworkEval, Relation, RelationEFTraitBound,
        RelationEntry, TraceLocationAllocator, ORIGINAL_TRACE_IDX,
    },
    core::{
        air::Component,
        backend::{
            simd::{
                m31::{PackedM31, LOG_N_LANES, N_LANES},
                qm31::PackedSecureField,
                SimdBackend,
            },
            Col, Column,
        },
        channel::{Blake2sChannel, Channel},
        fields::{
            m31::{BaseField, M31},
            qm31::SecureField,
        },
        pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig},
        poly::{
            circle::{CanonicCoset, CircleEvaluation, PolyOps},
            BitReversedOrder,
        },
        prover::{prove, verify, StarkProof},
        vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
        ColumnVec,
    },
};

// ANCHOR: read_write_memory_component
pub type ReadWriteMemoryComponent = FrameworkComponent<ReadWriteMemoryEval>;
// ANCHOR_END: read_write_memory_component

// ANCHOR: memory_op
/// Represents a single memory operation (read or write)
#[derive(Default, Clone, Debug)]
pub struct MemoryOp {
    /// Flag indicating if this is a read (0) or write (1) operation
    pub rw_flag: M31,
    /// Memory address being accessed
    pub addr: M31,
    /// Value being read or written
    pub value: M31,
    /// Operation counter for ordering
    pub counter: M31,
}

impl MemoryOp {
    pub fn new(rw_flag: u32, addr: u32, value: u32, counter: u32) -> Self {
        Self {
            rw_flag: M31::from(rw_flag),
            addr: M31::from(addr),
            value: M31::from(value),
            counter: M31::from(counter),
        }
    }
}
// ANCHOR_END: memory_op

// ANCHOR: constants
/// Number of columns used for a memory operation
pub const MEM_OP_COL_NUM: usize = 4;
/// Total number of columns in the trace
pub const TRACE_COLUMN_SIZE: usize = MEM_OP_COL_NUM * 2;
// ANCHOR_END: constants

// ANCHOR: constraint_log_expand
pub const CONSTRAINT_LOG_EXPAND: u32 = 1;
// ANCHOR_END: constraint_log_expand

// ANCHOR: lookup_elements
/// Random values used for random linear combination of `MEM_OP_COL_NUM` elements
#[derive(Debug, PartialEq)]
pub struct ReadWriteMemoryLookupElements {
    pub lookup_elements: LookupElements<MEM_OP_COL_NUM>,
}

impl ReadWriteMemoryLookupElements {
    /// Creates new lookup elements by drawing from the channel
    pub fn draw(channel: &mut impl Channel) -> Self {
        let lookup_elements = LookupElements::draw(channel);
        Self { lookup_elements }
    }
}

impl<F: Clone + std::fmt::Debug, EF: RelationEFTraitBound<F>> Relation<F, EF>
    for ReadWriteMemoryLookupElements
{
    fn combine(&self, values: &[F]) -> EF {
        self.lookup_elements.combine(values)
    }

    fn get_name(&self) -> &str {
        "ReadWriteMemoryLookupElements"
    }

    fn get_size(&self) -> usize {
        MEM_OP_COL_NUM
    }
}
// ANCHOR_END: lookup_elements

// ANCHOR: eval
#[derive(Debug)]
pub struct ReadWriteMemoryEval {
    pub log_size: u32,
    pub lookup_elements: ReadWriteMemoryLookupElements,
}

impl FrameworkEval for ReadWriteMemoryEval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        evaluate_constraints(&mut eval, &self.lookup_elements);
        eval
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + CONSTRAINT_LOG_EXPAND
    }
}
// ANCHOR_END: eval

// ANCHOR: lookup_data
#[derive(Debug)]
pub struct LookupData {
    pub memory_ops: [Vec<PackedM31>; MEM_OP_COL_NUM],
    pub ordered_memory_ops: [Vec<PackedM31>; MEM_OP_COL_NUM],
}
// ANCHOR_END: lookup_data

// ANCHOR: evaluate_constraints
fn evaluate_constraints<E: EvalAtRow>(
    eval: &mut E,
    lookup_elements: &ReadWriteMemoryLookupElements,
) {
    // Get original memory operation fields
    let [rw_flag] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [addr] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [value] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [counter] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);

    let memory_op: [_; MEM_OP_COL_NUM] = [rw_flag, addr, value, counter];

    // Get ordered memory operation fields
    let [rw_flag] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [addr] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [value] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);
    let [counter] = eval.next_interaction_mask(ORIGINAL_TRACE_IDX, [0]);

    let ordered_memory_op: [_; MEM_OP_COL_NUM] = [rw_flag, addr, value, counter];

    eval.add_to_relation(RelationEntry::new(
        lookup_elements,
        E::EF::one(),
        &memory_op,
    ));
    eval.add_to_relation(RelationEntry::new(
        lookup_elements,
        -E::EF::one(),
        &ordered_memory_op,
    ));

    eval.finalize_logup_in_pairs();
}
// ANCHOR_END: evaluate_constraints

/// Generates a proof for a sequence of memory operations
///
/// # Arguments
///
/// * `memory_ops` - Vector of memory operations to prove
/// * `log_size` - Log size of the trace
///
/// # Returns
///
/// A tuple containing the memory component and the STARK proof
// ANCHOR: prove_read_write_memory_fn_start
pub fn prove_read_write_memory(
    memory_ops: Vec<MemoryOp>,
    log_size: u32,
) -> Result<(ReadWriteMemoryComponent, StarkProof<Blake2sMerkleHasher>), &'static str> {
    // ANCHOR_END: prove_read_write_memory_fn_start
    // ANCHOR: setup
    let config = PcsConfig::default();

    // Precompute twiddles for FFT
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_size + CONSTRAINT_LOG_EXPAND + config.fri_config.log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    // Define the commitment scheme used to commit to the trace
    let mut commitment_scheme =
        CommitmentSchemeProver::<_, Blake2sMerkleChannel>::new(config, &twiddles);

    // Define the channel used to draw random elements from a digest.
    let channel = &mut Blake2sChannel::default();
    // ANCHOR_END: setup

    // ANCHOR: commit_trace
    // Generate and commit preprocessed trace
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(vec![]);
    tree_builder.commit(channel);

    // Generate and commit original trace
    let (trace, lookup_data) = gen_trace(memory_ops, log_size);
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(trace);
    tree_builder.commit(channel);
    // ANCHOR_END: commit_trace

    // ANCHOR: draw_lookup_elements
    // Draw lookup elements and generate interaction trace
    let lookup_elements = ReadWriteMemoryLookupElements::draw(channel);
    // ANCHOR_END: draw_lookup_elements

    // ANCHOR: commit_interaction_trace
    let (interaction_trace, claimed_sum) =
        gen_interaction_trace(log_size, lookup_data, &lookup_elements);
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(interaction_trace);
    tree_builder.commit(channel);
    // ANCHOR_END: commit_interaction_trace

    // ANCHOR: create_component
    // Prove constraints
    let component = ReadWriteMemoryComponent::new(
        &mut TraceLocationAllocator::default(),
        ReadWriteMemoryEval {
            log_size,
            lookup_elements,
        },
        claimed_sum,
    );
    // ANCHOR_END: create_component

    // ANCHOR: prove
    let proof =
        prove(&[&component], channel, commitment_scheme).map_err(|_| "Failed to generate proof")?;
    // ANCHOR_END: prove

    Ok((component, proof))
    // ANCHOR: prove_read_write_memory_fn_end
}
// ANCHOR_END: prove_read_write_memory_fn_end

// ANCHOR: gen_trace
fn gen_trace(
    memory_ops: Vec<MemoryOp>,
    log_size: u32,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    LookupData,
) {
    let mut trace = (0..TRACE_COLUMN_SIZE)
        .map(|_| Col::<SimdBackend, BaseField>::zeros(1 << log_size))
        .collect_vec();

    for vec_index in 0..(1 << (log_size - LOG_N_LANES)) {
        trace[0].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| memory_ops[vec_index * N_LANES + i].rw_flag)
                .try_into()
                .unwrap(),
        );
        trace[1].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| memory_ops[vec_index * N_LANES + i].addr)
                .try_into()
                .unwrap(),
        );
        trace[2].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| memory_ops[vec_index * N_LANES + i].value)
                .try_into()
                .unwrap(),
        );
        trace[3].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| memory_ops[vec_index * N_LANES + i].counter)
                .try_into()
                .unwrap(),
        );
    }

    // order trace based on address and counter
    let mut ordered_memory_ops = memory_ops.clone();
    ordered_memory_ops.sort_by(|a, b| {
        // First sort by addr
        let addr_cmp = a.addr.cmp(&b.addr);
        if addr_cmp == std::cmp::Ordering::Equal {
            // If addresses are equal, sort by counter
            a.counter.cmp(&b.counter)
        } else {
            addr_cmp
        }
    });

    for vec_index in 0..(1 << (log_size - LOG_N_LANES)) {
        trace[MEM_OP_COL_NUM].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| ordered_memory_ops[vec_index * N_LANES + i].rw_flag)
                .try_into()
                .unwrap(),
        );
        trace[MEM_OP_COL_NUM + 1].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| ordered_memory_ops[vec_index * N_LANES + i].addr)
                .try_into()
                .unwrap(),
        );
        trace[MEM_OP_COL_NUM + 2].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| ordered_memory_ops[vec_index * N_LANES + i].value)
                .try_into()
                .unwrap(),
        );
        trace[MEM_OP_COL_NUM + 3].data[vec_index] = PackedM31::from_array(
            std::array::from_fn(|i| ordered_memory_ops[vec_index * N_LANES + i].counter)
                .try_into()
                .unwrap(),
        );
    }

    let lookup_data = LookupData {
        memory_ops: std::array::from_fn(|i| trace[i].data.clone()),
        ordered_memory_ops: std::array::from_fn(|i| trace[i + MEM_OP_COL_NUM].data.clone()),
    };

    let domain = CanonicCoset::new(log_size).circle_domain();
    let trace = trace
        .into_iter()
        .map(|eval| CircleEvaluation::new(domain, eval))
        .collect();

    (trace, lookup_data)
}
// ANCHOR_END: gen_trace

// ANCHOR: gen_interaction_trace
fn gen_interaction_trace(
    log_size: u32,
    lookup_data: LookupData,
    lookup_elements: &ReadWriteMemoryLookupElements,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    SecureField,
) {
    let mut logup_gen = LogupTraceGenerator::new(log_size);

    let mut col_gen = logup_gen.new_col();

    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let unordered_memory_ops = lookup_data.memory_ops.each_ref().map(|s| s[vec_row]);
        let denom0: PackedSecureField = lookup_elements.combine(&unordered_memory_ops);

        let ordered_memory_ops = lookup_data
            .ordered_memory_ops
            .each_ref()
            .map(|s| s[vec_row]);
        let denom1: PackedSecureField = lookup_elements.combine(&ordered_memory_ops);

        col_gen.write_frac(vec_row, denom1 - denom0, denom1 * denom0);
    }

    col_gen.finalize_col();

    logup_gen.finalize_last()
}
// ANCHOR_END: gen_interaction_trace

fn verify_read_write_memory(
    component: ReadWriteMemoryComponent,
    proof: StarkProof<Blake2sMerkleHasher>,
) {
    // Setup
    let channel = &mut Blake2sChannel::default();
    let config = PcsConfig::default();
    let commitment_scheme = &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

    // Retrieve the expected column sizes in each commitment interaction, from the AIR.
    let sizes = component.trace_log_degree_bounds();

    // Preprocessed columns.
    commitment_scheme.commit(proof.commitments[0], &sizes[0], channel);

    // Trace columns.
    commitment_scheme.commit(proof.commitments[1], &sizes[1], channel);

    // Draw lookup element.
    let lookup_elements = ReadWriteMemoryLookupElements::draw(channel);
    assert_eq!(lookup_elements, component.lookup_elements);

    // Interaction columns.
    commitment_scheme.commit(proof.commitments[2], &sizes[2], channel);

    verify(&[&component], channel, commitment_scheme, proof).unwrap();
}

// ANCHOR: main_start
fn main() {
    // ANCHOR_END: main_start
    // ANCHOR: main_create_memory_ops
    let log_size = 4;

    let mut memory_ops = vec![
        MemoryOp::new(0, 1, 1, 0),
        MemoryOp::new(1, 2, 2, 1),
        MemoryOp::new(1, 3, 0, 2),
        MemoryOp::new(1, 3, 1, 3),
        MemoryOp::new(0, 2, 2, 4),
        MemoryOp::new(0, 1, 1, 5),
    ];

    // Pad with dummy operations
    let memory_ops_len = memory_ops.len() as u32;
    memory_ops.extend(
        (0..((1 << log_size) - memory_ops_len))
            .map(|i| MemoryOp::new(0, (1 << log_size) - 1, 0, i + memory_ops_len)),
    );
    // ANCHOR_END: main_create_memory_ops

    // ANCHOR: main_prove
    let (component, proof) = prove_read_write_memory(memory_ops, log_size as u32).unwrap();
    // ANCHOR_END: main_prove
    verify_read_write_memory(component, proof);
    // ANCHOR: main_end
}
// ANCHOR_END: main_end
