# Cairo

Let's start by understanding how Cairo works. If you would like to learn more formally about Cairo, you can refer to the original [Cairo paper](https://eprint.iacr.org/2021/1063.pdf).

Essentially, Cairo is a CPU architecture that is Turing-complete and is specifically designed so that it can also be used to create efficient proofs of execution using STARKs. In particular, Cairo uses the read-only memory model instead of the more common read-write memory model and only uses a relatively small number of registers.

## Non-Deterministic Read-Only Memory

A read-only memory model is a memory model where each address in the memory can only have a single value throughout the execution of the program. This is in contrast to the more common read-write memory model where an address can have multiple values at different points in time.

The memory is also non-deterministic, which means that the prover provides the values of the memory cells as a witness and does not provide any additional constraints.

## Registers

In physical CPUs, accessing the memory is expensive compared to accessing registers due to physical proximity. This is why instructions typically operate over registers rather than directly over the memory cells. Since accessing the memory and accessing registers is the same in Cairo, however, Cairo instructions operate directly over memory cells. Thus, the 3 registers used in Cairo do not store arbitrary values but rather pointers to the memory cells:

- `pc` is the **program counter**, which points to the current Cairo instruction
- `ap` is the **allocation pointer**, which points to the current available memory address
- `fp` is the **frame pointer**, which points to the current frame in the "call stack"

[TODO: add a simple example]

## Cairo Instructions

Let's see what a Cairo instruction looks like.

<figure id="fig-cairo-instruction" style="text-align: center;">
    <img src="./cairo-instruction.png" width="100%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 1: Cairo instruction (little-endian)</span></center></figcaption>
</figure>

As the figure above from the [Cairo paper](https://eprint.iacr.org/2021/1063.pdf) shows, an instruction is made up of 64 bits, where the first 3 16-bit integers are signed offsets to the 3 operands `dst`, `op0`, and `op1`.

The next 15 bits are flags, where the `dst_reg` and `op0_reg` indicates whether to use the `ap` or the `fp` register value as the base value for the `dst` and the `op0` operands, respectively. The `op1_src` flag support 4 different values as the base values for the `op1` operand: `op0`, `pc`, `fp`, and `ap`. The `res_logic` flag indicates how to compute the operands: `op1`, `op0 + op1`, or `op0 * op1`. The `pc_update` and `ap_update` flags show how to update the `pc` and `ap` registers, respectively, after computing the operands and the `opcode` flag indicates whether this instruction belongs to a predefined opcode (e.g. `CALL`, `RET`, `ASSERT_EQ`) or not and also defines how the `ap` and `fp` registers should be updated.

Finally the last bit is fixed to 0, but as we will see later, this design is modified in the current version of Cairo to support opcode extensions.

[TODO: add a simple example]

### Opcodes and Opcode Extensions

In Cairo, an opcode refers to what the instruction should do. Since there is a strict set of constraints that an instruction should follow, every instruction can be seen as a _generic_ opcode, but Cairo also defines specific opcodes for commonly used functions (e.g. `ADD`, `MUL`, `JUMP`, `CALL`, etc).

In addition to the opcodes defined in the Cairo paper, however, Cairo now also defines a new set of opcodes using **opcode extension**. As you can see in the figure below, this is done by extending the size of an instruction from 64 bits to 72 bits and using the value of the last 9 bits.

<figure id="fig-opcode-extension" style="text-align: center;">
    <img src="./opcode-extension.png" width="100%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 2: Opcode extension bits</span></center></figcaption>
</figure>

In the previous section, we mentioned that the last bit of a Cairo instruction was fixed to 0, but this is no longer the case. If the value of the opcode extension is 0, then we use the original set of opcodes, but if it is not, we check if it matches to any of the new extended opcodes. As of [this commit](https://github.com/starkware-libs/stwo-cairo/tree/b712c77887f8f8ce0d39a8a9741221c89846836e), the following opcode extension values are supported:

- `0`: Stone (original opcodes)
- `1`: Blake
- `2`: BlakeFinalize
- `3`: QM31Operation
