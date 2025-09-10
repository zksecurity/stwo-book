# Additional Examples

Here, we introduce some additional AIRs that may help in designing more complex AIRs.

## Selectors

A selector is a column of 0s and 1s that is used to selectively enable or disable a constraint. One example of a selector is the `IsFirst` column which has a value of 1 only on the first row. This can be used when constraints are defined over both the current and previous rows but we need to make an exception for the first row.

For example, as seen in [Figure 1](#fig-selectors), when we want to track the cumulative sum of a column, i.e. \\(b_2 = a_1 + a_2\\), the previous row of the first row will point to the last row, creating an incorrect constraint \\(a_1 = b_4 + b_1\\). Thus, we need to disable the constraint for the first row and enable a separate constraint \\(a_1 = b_1\\). This can be achieved by using a selector column that has a value of 1 on the first row and 0 on the other rows and multiplying the constraint by the selector column:

$$
(1 - \text{IsFirst(X)}) \cdot (A(\omega \cdot X) - B(X) - B(\omega \cdot X)) + \text{IsFirst(X)} \cdot (A(X) - B(X)) = 0
$$

where \\(X\\) refers to the previous value of \\(\omega\cdot X\\) in the multiplicative subgroup of the finite field.

<figure id="fig-selectors" style="text-align: center;">
    <img src="./selectors.png" width="50%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 1: IsFirst selector</span></center></figcaption>
</figure>

## IsZero

Checking that a certain field element is zero is a common use case when writing AIRs. To do this efficiently, we can use the property of finite fields that a non-zero field element always has a multiplicative inverse.

For example, in [Figure 2](#fig-is-zero), we want to check that a field element in \\(\mathbb{F}\_5\\) is zero. We create a new column that contains the multiplicative inverse of each field element \\(a_i\\). We then use the multiplication of the two columns and check whether the result is 0 or 1. Note that if the existing column has a zero element, we can insert any value in the new column since the multiplication will always be zero.

This way, we can create a constraint that uses the `IsZero` condition as part of the constraint, e.g. \\((1 - (A(X) \cdot Inv(X))) \cdot (\text{constraint_1}) + (A(X) \cdot Inv(X)) \cdot (\text{constraint_2}) = 0\\), which checks \\(\text{constraint_1}\\) if \\(A(X)\\) is 0 and \\(\text{constraint_2}\\) if \\(A(X)\\) is not 0.

<figure id="fig-is-zero" style="text-align: center;">
    <img src="./is-zero.png" width="40%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 2: IsZero over a prime field 5</span></center></figcaption>
</figure>

## Public Inputs

When writing AIRs, we may want to expose some values in the trace to the verifier to check in the open. For example, when running an AIR for a Cairo program, we may want to check that the program that was executed is the correct one.

In Stwo, we can achieve this by adding the public input portion of the trace as a LogUp column as negative multiplicity. As shown in [Figure 3](#fig-public-inputs), the public inputs \\(a_1, a_2\\) are added as LogUp values with negative multiplicity \\(\dfrac{-1}{X - a_1}\\) and \\(\dfrac{-1}{X - a_2}\\). The public inputs are given to the verifier as part of the proof and the verifier can directly compute the LogUp values with positive multiplicity \\(\dfrac{1}{X - a_1}\\) and \\(\dfrac{1}{X - a_2}\\) and add it to the LogUp sum and check that the total sum is 0.

<figure id="fig-public-inputs" style="text-align: center;">
    <img src="./public-inputs.png" width="70%" />
    <figcaption><center><span style="font-size: 0.9em">Figure 3: Public inputs</span></center></figcaption>
</figure>

## XOR

🚧
