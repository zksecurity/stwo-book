# Basis for Circle FFT

The circle FFT algorithm outputs coefficients \\( c_j \\) with respect to some basis \\( b_j^{(n)}(x, y) \\) such that:
$$
p(x, y) = \sum_{j = 0}^{2^n - 1} c_j \cdot b_j^{(n)}(x, y)
$$

In the concrete example of Circle FFT over twin-coset \\( D_3 \\), we saw that the algorithm computed the coefficient with respect to the following basis:

$$b^{(3)}_j(x, y) = [1, x, \pi(x), x \cdot \pi(x), y, y \cdot x, y \cdot \pi(x), y \cdot x \cdot \pi(x)]$$

Using induction on \\( n \\), we can show that the Circle FFT algorithm outputs coefficients with respect to the following basis [[Theorem 2, Circle STARKs](https://eprint.iacr.org/2024/278.pdf)]:

$$
b^{(n)}_j(x, y) := y^{j_0} \cdot x^{j_1} \cdot \pi(x)^{j_2} \cdot \pi^2(x)^{j_3} \cdots \pi^{n-2}(x)^{j\_{n-1}}
$$
 
where \\( 0 \leq j \leq 2^n - 1 \\) and \\( (j_0, \ldots, j_{n-1}) \in \\{0, 1\\}^n \\) is the binary representation of \\(j\\), i.e., 

$$
j = j_0 + j_1 \cdot 2 + \cdots + j_{n-1} \cdot 2^{n-1}
$$ 

# Dimension Gap
Let the space spanned by the basis polynomials in \\(b^{(n)}(x, y)\\) be \\( L'_N(\mathbb{F}) \\). The basis \\(b^{(n)}(x, y)\\) has a total \\( N=2^n \\) elements and thus the dimension of the space \\( L'_N(\mathbb{F}) \\) is \\( N \\). However, the space of bivariate polynomials over the circle curve is \\( L_N(\mathbb{F}) \\) which has dimension \\( N+1 \\).

We can identify the missing highest total degree element in \\( L'_N(\mathbb{F}) \\) by examining the basis. The highest total degree element in basis \\(b^{(n)}(x, y)\\) is:
$$y \cdot x \cdot \pi(x) \cdot \pi^2(x) \cdot \pi^3(x) \cdots \pi^{n-2}(x)$$

Using \\( deg(\pi^j(x) = 2^j) \\), the highest degree of \\( X \\) in the above term is:
$$1 + 2 + 2^2 + 2^3 + \cdots + 2^{n-2} = 2^{n-1} - 1 = N/2 - 1$$

Since the highest degree of \\( X \\) in \\( L'_N(\mathbb{F}) \\) is \\( N/2 - 1 \\), we can represent the space \\( L'_N(\mathbb{F}) \\) as follows:
$$
L'_N(\mathbb{F}) = \mathbb{F}[X]^{ \leq N/2 - 1} + Y \cdot \mathbb{F}[X]^{ \leq N/2 - 1} 
$$
where \\( \mathbb{F}[X]^{ \leq N/2 - 1} \\) represents polynomials of degree at most \\( N/2 - 1 \\) with coefficients in $\mathbb{F}$. Similarly, we can represent the space \\( L_N(\mathbb{F}) \\) as:
$$
L_N(\mathbb{F}) = \mathbb{F}[X]^{ \leq N/2} + Y \cdot \mathbb{F}[X]^{ \leq N/2 - 1} 
$$

Thus the space \\( L'_N(\mathbb{F}) \\) does not include the monomial \\( X^{N/2} \\), which lies in the space \\( L_N(\mathbb{F}) \\). Therefore,
$$
L_N(\mathbb{F}) = L'_N(\mathbb{F}) + \langle X^{N/2} \rangle
$$

Since the space spanned by \\( X^{N/2} \\) is same as the space spanned by the vanishing polynomial \\( v_n(x) \\) which has degree \\( deg(v_n) = 2^{n-1} =N/2 \\), we can also write:
$$
L_N(\mathbb{F}) = L'_N(\mathbb{F}) + \langle v_n \rangle
$$

A consequence of this dimension gap is that we cannot interpolate some polynomials over the circle i.e. those with \\( X^{N/2} \\). We will address how this dimension gap is handled within the FRI protocol in the upcoming sections.