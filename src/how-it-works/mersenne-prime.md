# Mersenne Primes

Proof systems typically rely on finite field operations, where efficient field arithmetic is crucial for optimizing proof generation. In STARK protocols, there is no direct dependency between the security level of the proof system and the field size. This allows the use of small fields with highly efficient arithmetic, such as Mersenne prime fields.

A Mersenne prime is defined as a prime number that is one less than a power of two, expressed as \\( p = 2^k -1 \\).

Consider the Mersenne prime field \\( \mathbb{F}_p \\) where \\( p = 2^{31} - 1 \\). Our objective is to perform field multiplication \\( a \cdot b \\), where \\( a, b \in \mathbb{F}_p \\). This operation involves a 31-bit integer multiplication, producing a 62-bit intermediate result, which is then reduced modulo \\( p \\).

Let \\( x = a \cdot b \\), where \\( a, b \\) are 31-bit values, resulting in a 62-bit product \\( x \\). We can decompose \\( x \\) into two 31-bit values \\( b \\) and \\( s \\), such that \\( x = 2^{31} \cdot b + s \\), as shown in the following figure.

<div style="text-align: center;">
    <img src="./mersenne-mult.svg" alt="Mersenne Prime Multiplication" width="400px">
</div>

To perform modular reduction, we start with:
\\[ x \equiv (2^{31} \cdot b + s) \quad mod \quad (2^{31} - 1) \\]
Substituting \\( 2^{31} \equiv 1 \mod (2^{31} - 1) \\) gives:
\\[ x \equiv (b + s) \quad mod \quad (2^{31} - 1) \\]

Since \\( b \\) and \\( s \\) are both 31-bit values, they can be directly represented as field elements. Consequently, modular reduction is performed with a single field addition. This makes arithmetic over Mersenne primes exceptionally fast, making them an ideal choice for our STARK protocol.

However, we instantiate STARK protocols over an FFT-friendly field, meaning a field that contains a multiplicative subgroup of order that is a large power of two (commonly referred to as a smooth subgroup).

\\[ |\mathbb{F}_p^*| = p-1 = 2^k-2\\]

As shown above, Mersenne prime fields lack a smooth subgroup of size that is a large power of two because there is no large power of two that divides \\( |\mathbb{F}\_{p}^*| \\). In other words, there does not exist a sufficiently large \\( n \\) such that \\( 2^n \\, | \\, p - 1 \\).


# Extensions of Mersenne Prime Field

To make Mersenne prime fields compatible with STARKs, we use a degree-2 extension of \\( \mathbb{F}_p \\), defined as follows:

\\[ \mathbb{F}\_{p^2} = \mathbb{F}_p[X]/(X^2 + 1) \\] 

This extension forms a field of size \\( p^2 \\), where elements can be represented as \\( (a, b) \\) or 
\\[ a + i \cdot b \\] 
where \\( a, b \in \mathbb{F}_p \\) and \\( i \\) is the root of the polynomial \\( X^2 + 1 \\) i.e. \\( i^2 + 1 = 0\\).

The order of the multiplicative group of this extended field is calculated as follows:

\\[ |\mathbb{F}_{p^2}^*|  = p^2 - 1 = (p-1) \cdot (p+1)\\]

For Mersenne primes of the form \\( p = 2^k - 1 \\), this becomes: 

\\[ |\mathbb{F}_{p^2}^*| = (2^k-2) \cdot (2^k)\\]

As shown above, \\( 2^k \\, | \\, |\mathbb{F}\_{p^2}^\*| \\) i.e. \\( \mathbb{F}_{p^2}^* \\) contains a subgroup of size that is a large power of two. This makes it suitable for instantiating STARKs. This subgroup is what we refer to as the Circle group (explored further in the next section).

## Secure Field 
For the soundness of the protocol, it is crucial that the verifier samples random challenges from a sufficiently large field to ensure that an adversary cannot guess or brute-force the challenges and generate a proof that passes verification without knowledge of the witness.

If we use \\( p = 2^{31} -1 \\), then 31-bit random challenges are not sufficient to maintain the security of the protocol. To address this, the verifier draws random challenges from a degree-4 extension of \\( \mathbb{F}\_{p} \\), which is equivalent to degree-2 extension of \\( \mathbb{F}\_{p^2} \\), denoted as 
\\[ \mathbb{F}\_{p^4} = \mathbb{F}\_{p^2}[X]/(X^2 - 2 - i) \\]

The elements of \\( \mathbb{F}\_{p^4} \\) can be represented as \\( (r, s) \\) or 
\\[ r + u \cdot s \\] 
where \\( r, s \in \mathbb{F}_{p^2} \\) and \\( u \\) is the root of the polynomial \\( X^2 - 2 - i \\) i.e. \\( u^2 - 2 - i = 0\\). 

Alternatively, the elements of \\( \mathbb{F}\_{p^4} \\) can also be represented as four elements of \\( \mathbb{F}\_{p} \\) i.e. \\( ((a, b), (c, d)) \\) or 
\\[ (a + i \cdot b) + (c + i \cdot d) \cdot u \\] 

where \\( a, b, c, d \in \mathbb{F}_p \\). With four elements from \\( \mathbb{F}\_{p} \\), the challenge space consists of 124-bit values, offering a sufficiently large \\( 2^{124} \\) possibilities to sample a random challenge.
