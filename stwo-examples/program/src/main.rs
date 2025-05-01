use stwo_custom_air::phase1::prove_read_write_memory as prove_read_write_memory_phase1;
use stwo_custom_air::phase1::MemoryOp as Phase1MemoryOp;

fn main() {
    prove_phase1();
}

fn prove_phase1() {
    let log_size = 6;

    let mut memory_ops = vec![
        Phase1MemoryOp::new(0, 1, 1, 0),
        Phase1MemoryOp::new(1, 2, 2, 1),
        Phase1MemoryOp::new(1, 3, 0, 2),
        Phase1MemoryOp::new(1, 3, 1, 3),
        Phase1MemoryOp::new(0, 2, 2, 4),
        Phase1MemoryOp::new(0, 1, 1, 5),
    ];

    // Pad with dummy operations
    let memory_ops_len = memory_ops.len() as u32;
    memory_ops.extend(
        (0..((1 << log_size) - memory_ops_len))
            .map(|i| Phase1MemoryOp::new(0, (1 << log_size) - 1, 0, i + memory_ops_len)),
    );

    let (_component, _proof) = prove_read_write_memory_phase1(memory_ops, log_size as u32).unwrap();
}
