# AIR Development

At its core, a proof system can prove the validity of a statement $C(x,w)=y$ where $C$ is a representation of some logic, while $x$ and $w$ are inputs and $y$ the output of said logic. In the case of Stwo, that representation should be an Algebraic Intermediate Representation (AIR), which, very simply put, refers to a set of polynomials with constraints (i.e. equations) defined over them. A very simple example to illustrate this would be to think of three polynomials $f_1(x)$, $f_2(x)$, and $f_3(x)$ with a constraint that $f_1(x)$ multiplied by $f_2(x)$ equals $f_3(x)$.

AIRs are not unique to Stwo and is widely used in various proof systems like Plonky3 and RISC Zero. In fact, they are even almost identical in construction with other intermediate representations like PLONK.

Yet, the real advantage of using Stwo is that it provides a fast prover by leveraging the Mersenne-31 prime field, where computations are done modulo $2^{31} - 1$. In general, using smaller prime fields leads to faster computations (e.g. Halo2 needs to use big fields like 254-bit prime fields), but the M31 field is special because it allows for a very efficient modular reduction. We suggest going through [this post](https://blog.zksecurity.xyz/posts/circle-starks-1/) for a breakdown of why this is the case.

In this section, we will introduce concepts around AIRs and show how to use Stwo abstractions to create custom AIRs optimized for specific use-cases. We will go through multiple examples to help you get started, so stay tuned!
