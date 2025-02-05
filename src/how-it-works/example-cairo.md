# Example: Cairo

This section walks through how Stwo processes a Cairo program to generate zero-knowledge proofs. The process consists of several key steps:

## 1. Compilation and VM Execution

First, the Cairo program is compiled into CASM (Cairo Assembly) and executed in the Cairo VM. This step generates the initial program execution trace.

## 2. Trace Generation

For each operation in the program execution, Stwo generates specific traces:

- Opcode execution traces
- Range check operation traces
- Memory access traces
- And other operation-specific traces

## 3. AIR Constraints with Lookup Tables

The system creates AIR (Algebraic Intermediate Representation) constraints using lookup tables. These constraints ensure the validity of:

- Program execution flow
- Memory operations
- Range checks
- Other program invariants

## 4. Polynomial Combination

Different lookup tables are combined via interaction into a single polynomial through:

- Random linear combination in the extended domain
- This step helps reduce the proof size and verification complexity

## 5. FRI Protocol

Finally, Stwo performs FRI (Fast Reed-Solomon Interactive Oracle Proof) on the final polynomial in the extended domain to generate the proof.
