# Proving and Verifying an AIR

<figure id="fig-proving-an-air-1">
    <img src="./proving-an-air-1.png" width="100%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 1: Prover workflow: perform FRI and PoW</span></center></figcaption>
</figure>

We're finally ready to take the final step--prove and verify an AIR!

Since the code is relatively short, let us present it first and then go over the details.

```rust,ignore
{{#include ../../../stwo-examples/examples/proving_an_air.rs:here_1}}
{{#include ../../../stwo-examples/examples/proving_an_air.rs:here_2}}
```

## Prove

As you can see, there is only a single line of code added to create the proof. The `prove` function performs the FRI and PoW operations under the hood, although technically, the constraint related steps in [Figure 1](#fig-proving-an-air-1) were not performed in the previous section and are only performed once `prove` is called.

## Verify

In order to verify our proof, we need to check that the constraints are satisfied using the commitments from the proof. In order to do that, we need to set up a `Blake2sChannel` and `CommitmentSchemeVerifier<Blake2sMerkleChannel>`, along with the same `PcsConfig` that we used when creating the proof. Then, we need to recreate the running hash channel by passing the merkle tree commitments and the `log_num_rows` to the `CommitmentSchemeVerifier` instance by calling `commit` (remember, the order is important!). Then, we can verify the proof using the `verify` function.

```admonish exercise
Try setting the dummy values in the table to 1 instead of 0. Does it fail? If so, can you see why?
```

Congratulations! We have come full circle. We now know how to create a table, convert it to trace polynomials, commit to them, create constraints over the trace polynomials, and prove and verify the constraints (i.e. an AIR). In the following sections, we will go over some more complicated AIRs to explain Stwo's other features.
