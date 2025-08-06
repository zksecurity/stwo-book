# Basic Building Blocks

This section will go over the basic building blocks that are used to build the Cairo AIR.

## Felt252 to M31

Cairo works over the prime field \\(P = 2^{251} + 17 \cdot 2^{192} + 1\\), while Stwo works over the prime field \\(M31 = 2^{31} - 1\\). Thus, in order to represent the execution of Cairo with Stwo, we need to decompose the 252-bit integers into 31-bit integers. The Cairo AIR chooses to use the 9-bit decomposition, so a single 252-bit integer will result in 28 9-bit limbs. To ensure that the decomposition is correct, we also need to verify that each limb is in the range \\(0 \leq \text{limb} < 2^{31}\\).
