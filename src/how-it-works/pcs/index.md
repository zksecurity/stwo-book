# Polynomial Commitment Scheme

> This section presents the implementation of the polynomial commitment scheme in Stwo, which is built on top of the FRI protocol described previously. Polynomial commitments are a core cryptographic primitive that enable a prover to commit to a polynomial and later reveal evaluations at specific points.

This section is organized as follows:

- [**Overview**](./overview.md): Describes the polynomial commitment scheme of Stwo.
- [**PCS Prover**](./prover.md): Details the implementation of the prover for the polynomial commitment scheme, including commitment and opening protocol.
- [**PCS Verifier**](./verifier.md): Describes the verifier implementation for checking commitments and evaluation proofs.