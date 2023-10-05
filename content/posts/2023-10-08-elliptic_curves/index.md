---
title: "What the hell is an Elliptic Curve?"
date: 2023-10-01T16:08:15-03:00
javascript: true
math: true
mermaid: false
---

> Warning: This post has [KaTeX](https://katex.org/) enabled,
> so if you want to view the rendered math formulas,
> you'll have to unfortunately enable JavaScript.
>
> Note: All elliptic curve images are made using $\LaTeX$ and TikZ.
> The source code is freely available at [GitHub](https://gist.github.com/realeinherjar/84061e78c85e08476f373e77c5a652ac).

What the hell is an Elliptic Curve?
In this post, I'll try to explain what an elliptic curve is,
and why it's so important in cryptography.
If you ever used an end-to-end encrypted messaging app,
or hold Bitcoin (or any other shitcoin, though I don't recommend it),
you've probably heard of elliptic curves.
It is important to know that elliptic curves are not only used in cryptography,
but also in many other fields of mathematics,
such as number theory, algebraic geometry, and topology.

My main argument is that elliptic curves are not that hard to understand.
Additionally, since encryption is ubiquitous in our daily lives,
**understanding them is important to be a well-informed citizen**.
I'll try to explain the concepts in a simple way,
but I'll assume you have some basic knowledge of algebra and math symbols.
Finally, I'll use the standard math notation,
so that when you read about elliptic curves in other places,
you'll be able to make connections more easily.

## What is an Elliptic Curve?

**An [_elliptic curve_](https://en.wikipedia.org/wiki/Elliptic_curve) is a curve defined by the equation**

$$y^2 = x^3 + ax + b$$

where $a$ and $b$ are real numbers.

Here's an example of an elliptic curve,
if you set $a = 0$ and $b = 7$:

$$y^2 = x^3 + 7$$

**This is [`secp256k1`](https://en.bitcoin.it/wiki/Secp256k1),
the notorious elliptic curve used in Bitcoin**.

{{< figure src="ec.png" title="secp256k1 curve" >}}

Elliptic curves have some properties that make them interesting.
For example, they are **symmetric with respect to the $x$-axis**.
This means that if $(x, y)$ is a point in the curve,
then $(x, -y)$ is also a point in the curve[^duality].
This is because $(-y)^2 = y^2$.

### Adding Points in an Elliptic Curve

If you have two points $P$ and $P^\prime$ in the curve,
then the line that passes through them will intersect the curve in a third point[^infinity]
$P^{\prime\prime}$:

{{< figure src="ec_point.png" title="Two Points in an Elliptic Curve" >}}

If you reflect $P^{\prime\prime}$ with respect to the $x$-axis,
you'll get a new point:

{{< figure src="ec_point_add.png" title="Adding Two Points in an Elliptic Curve" >}}

This is how you **add two points in an elliptic curve**.
Note that the result of adding two points is **also a point in the curve**.
This property is called _closure_.
For any two points $P$ and $P^\prime$ in the curve,
$P + P^\prime$ is also a point in the curve.

There are **two cases where the line doesn't intersect the curve in a third point**.
The first one is when $P = P^\prime$.

{{< figure src="ec_point_tangent.png" title="Adding a Point to Itself in an Elliptic Curve" >}}

Then we have a tangent line.
In this case we take derivatives on both sides:

$$\begin{aligned}
y^2 &= x^3 + ax + b \\\
2y \frac{dy}{dx} &= 3x^2 + a \\\
\frac{dy}{dx} &= \frac{3x^2 + a}{2y}
\end{aligned}$$

Hence, $2*P = (x, y)$ where $x = \left( \frac{3x^2 + a}{2y} \right) - 2x$,
and $y = \left( \frac{3x^2 + a}{2y} \right) (x - x) - y$.

The other is when $P^\prime$ is the reflection of $P$ with respect to the $x$-axis.

{{< figure src="ec_point_vertical.png" title="Adding a Point to Its Reflection in an Elliptic Curve" >}}

Then we have a vertical line.
In this case we define $P + P^\prime = \mathcal{O}$,
where $\mathcal{O}$ is the **point at infinity**.

### Point Addition Properties

Point addition satisfies some properties:

- **Identity**: $P + \mathcal{O} = \mathcal{O} + P = P$
- **Commutativity**: $P + P^\prime = P^\prime + P$
- **Associativity**: $(P + P^\prime) + P^{\prime\prime} = P + (P^\prime + P^{\prime\prime})$
- **Inverse Element**: $P + (-P) = \mathcal{O}$
- **Closure**: $P + P^\prime$ is a point in the curve

Hey this is very similar to addition of integers!
But using points instead of integers.

In fact, we can define **multiplication** of a point $P$ by an integer $n$ as:

$$\underbrace{P + P + \cdots + P}_{n \text{ times}}$$

### Abelian Groups

The set of points in an elliptic curve with the point at infinity $\mathcal{O}$
along with a binary operation $+$ that satisfies the properties above is called an
[_Abelian group_](https://en.wikipedia.org/wiki/Abelian_group).
In honor of Niels Henrik Abel, an Abelian group is a **set with a binary operation that
satisfies all the properties above**.

Another example of an Abelian group is the set of integers $\mathbb{Z}$ with addition $+$.
And another one is the set of natural numbers $\mathbb{N}$ with multiplication $\times$.

## Fields

There's one more concept that we need to learn before we can talk about cryptography.
It's called a finite field.
But before we talk about finite fields,
we need to first define a **field**.
**[Fields](https://en.wikipedia.org/wiki/Field_(mathematics)) are sets with two binary operations,
called addition $+$ and multiplication $\times$**.
We write

$$F = (F, +, \times)$$

to denote a field,
where $F$ is the set, $+$ is the addition operation,
and $\times$ is the multiplication operation.

Addition and multiplication behave similar to the addition and multiplication of real numbers.
For example, addition is **commutative** and **associative**

$$a + b = b + a,$$

and multiplication is **distributive**

$$a \times (b + c) = a \times b + a \times c.$$

Also, there are two special elements in the field,
called the **additive identity** $-a$ and the **multiplicative identity** $a^{-1}$,
such that

$$a + (-a) = I,$$

and

$$a \times a^{-1} = I,$$

where $I$ is the identity element.

Note that this allows us to define **subtraction**

$$a - b = a + (-b),$$

and **division**

$$a \div b = a \times b^{-1}.$$

### Finite Fields

Now we are ready for finite fields.
A [_finite field_](https://en.wikipedia.org/wiki/Finite_field), also called a Galois field (in honor of Évariste Galois),
is a **field with a finite number of elements.
As with any field, a finite field is a set on which the operations of multiplication,
addition, subtraction and division are defined and satisfy the rules above for fields**.

Finite fields is a very rich topic in mathematics,
and there are many ways to construct them.
The easiest way to construct a finite field is to take the **integers modulo a prime number $p$**.
For example $\mathbb{Z}_4$ is a finite field with 4 elements:

$$\mathbb{Z}_4 = \lbrace 0, 1, 2, 3 \rbrace.$$

In general, $\mathbb{Z}_n$ is a finite field with $n$ elements:

$$\mathbb{Z}_n = \lbrace 0, 1, 2, \ldots, n - 1 \rbrace.$$

**The number of elements in a finite field is called the _order_ of the field**.
The order of a finite field is **always a prime number $p$**.
The $\mathbb{Z}_4$ example above is a finite field of order 2.
To see that notice that $4$ is a composite number,

$$4 = 2 \times 2.$$

And we can write $\mathbb{Z}_4$ as

$$\mathbb{Z}_4 = 2 \times \mathbb{Z}_2.$$

This means that every element in $a \in \mathbb{Z}_4$ can be written as

$$a = 2 \times b,$$

where $b$ is an element in $\mathbb{Z}_2$.

Hence, not every element of $\mathbb{Z}_4$ is unique, and they are equivalent to the elements in $\mathbb{Z}_2$.

In general if $n$ is a composite number,
then $\mathbb{Z}_n$ is not a finite field.
However, if $n = r \times s$ where $r$ and $s$ are prime numbers,
and $r < s$,
then $\mathbb{Z}_n$ is a finite field of order $r$.

#### Operations in Finite Fields

**Addition** in finite fields is defined as the remainder of the sum of two elements modulo the order of the
field.

For example, in $\mathbb{Z}_3$,

$$1 + 2 = 3 \mod 3 = 0.$$

We can also define subtraction in finite fields as the remainder of the difference of two elements modulo the order of the field.

For example, in $\mathbb{Z}_3$,

$$1 - 2 = -1 \mod 3 = 2.$$

Multiplication in finite fields can be written as multiple additions.
For example, in $\mathbb{Z}_3$,

$$2 \times 2 = 2 + 2 = 4 \mod 3 = 1.$$

Exponentiation in finite fields can be written as multiple multiplications.
For example, in $\mathbb{Z}_3$,

$$2^2 = 2 \times 2 = 4 \mod 3 = 1.$$

This is very trivial for any finite field.
However, for division we are pretty much screwed.
It is really hard to find the multiplicative inverse of an element in a finite field.
For example, suppose that we have numbers $a,b$ in a very large finite field $\mathbb{Z}_n$,
such that

$$c = a \times b \mod n.$$

Then we can write division as

$$a = c \div b = c \times b^{-1} \mod n.$$

Now we need to find $b^{-1}$.
This is called the [**_discrete logarithm problem_**](https://en.wikipedia.org/wiki/Discrete_logarithm).
Because we need to find the exponent $b^{-1}$ such that

$$b^{-1} = \log_b c \mod n.$$

Since this number is a discrete number and not a real number,
that's why it's called the discrete logarithm problem.

Good luck my friend, no efficient method is known for computing them in general.
You can try brute force, but that's not efficient.

##### Why the Discrete Logarithm Problem is Hard as Fuck

To get a feeling why the discrete logarithm problem is difficult,
let's add one more concept to our bag of knowledge.
Every finite field has _**generators**_,
also known as _**primitive roots**_,
which is also a member of the group,
such that applying multiplication to this one single element
makes possible to generate the whole finite field.

Let's illustrate this with an example.
Below we have a table of all the results of the following operation

$$b^x \mod 7$$

for every possible value of $x$.
As you've guessed right this is the $\mathbb{Z}_7$ finite field.

| $b$ | $b^1 \mod 7$ | $b^2 \mod 7$ | $b^3 \mod 7$ | $b^4 \mod 7$ | $b^5 \mod 7$ | $b^6 \mod 7$ |
| :-: | :----------: | :----------: | :----------: | :----------: | :----------: | :----------: |
| $1$ |     $1$      |     $1$      |     $1$      |     $1$      |     $1$      |     $1$      |
| $2$ |     $2$      |     $4$      |     $1$      |     $2$      |     $4$      |     $1$      |
| $3$ |     $3$      |     $2$      |     $6$      |     $4$      |     $5$      |     $1$      |
| $4$ |     $4$      |     $2$      |     $1$      |     $4$      |     $2$      |     $1$      |
| $5$ |     $5$      |     $4$      |     $6$      |     $2$      |     $3$      |     $1$      |
| $6$ |     $6$      |     $1$      |     $6$      |     $1$      |     $1$      |     $1$      |

You see that something interesting is happening here.
For specific values of $b$, such as $b = 3$, and $b = 5$, we are able to **generate the whole finite field**.
Hence, say that $3$ and $5$ are _**generators**_ or _**primitive roots**_ of $\mathbb{Z}_7$.

Now suppose I ask you to find $x$ in the following equation

$$3^x \mod p = 11$$

where $p$ is a very large prime number.
Then you don't have any other option than brute forcing it.
**You'll need to try each exponent $x$ until you find the one that satisfies the equation**.

Notice that this operation is very asymmetric.
It is very easy to compute $3^x \mod p$ for any $x$,
but it is very hard to find $x$ given $3^x \mod p$.

## Bringing it All Together

Now we are ready to talk about elliptic curve cryptography.
**Elliptic curve cryptography is a public-key encryption technique based on elliptic curves**.
It is used to create public and private keys for asymmetric cryptography.

Your **private key is a random number $k$**.
Your **public key is the point $kG$ in the elliptic curve**,
where $G$ is a point and a generator of the elliptic curve.
We know that $kG$ is a point in the elliptic curve,
because elliptic curves are Abelian groups,
having the closure property,
then

$$kG = \underbrace{G + G + \cdots + G}_{k \text{ times}}$$

is also a point in the elliptic curve.

Now here comes the grand finale.
Since elliptic curves are finite fields,
then we can use the discrete logarithm problem to our advantage.
**It is very easy to compute $kG$ given $k$ and $G$**.
But it is **hard as fuck to find $k$ given $kG$ and $G$**.

## The `secp256k1` Elliptic Curve

Let's talk about some facts[^seg] about our muse `secp256k1` elliptic curve:

- **Equation**: $y^2 = x^3 + 7$
- **$\mathbb{F}_p$ where $p$**: $2^{256} - 2^{32} - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1$ (yes, it is a prime number)
- **Order $n$**: this is the number of possible points in the curve[^howtofind], 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141[^bignumber]
- **Generator Point $G$**: (0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798, 0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8)[^bignumber]

## License

This post is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International][cc-by-nc-sa].

[![CC BY-NC-SA 4.0][cc-by-nc-sa-image]][cc-by-nc-sa]

[cc-by-nc-sa]: http://creativecommons.org/licenses/by-nc-sa/4.0/
[cc-by-nc-sa-image]: https://licensebuttons.net/l/by-nc-sa/4.0/88x31.png

[^duality]: This is also why transaction malleability was a problem before Segwit in Bitcoin.
[^infinity]: If the line is vertical or tangent to the curve,
then it intersects the curve in a point at infinity.
[^seg]: These are standard and defined in the [Standards for Efficient Cryptography Group](https://www.secg.org/)
in their [SEC 2 specification](https://www.secg.org/sec2-v2.pdf).
[^howtofind]: To find the order of an elliptic curve, you can use [Schoof's algorithm](https://en.wikipedia.org/wiki/Schoof%27s_algorithm#The_algorithm).
[^bignumber]: It is a fucking big number, so I am putting the values in hexadecimal representation.
