---
title: "What the hell is an Elliptic Curve?"
date: 2023-10-01T16:08:15-03:00
draft: true
math: true
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
Additionally, since encryption is ubiquous in our daily lives,
**understanding them is important to be a well-informed citizen**.
I'll try to explain the concepts in a simple way,
but I'll assume you have some basic knowledge of algebra and math symbols.
Finally, I'll use the standard math notation,
so that when you read about elliptic curves in other places,
you'll be able to make connections more easily.

## What is an Elliptic Curve?

**An [*elliptic curve*](https://en.wikipedia.org/wiki/Elliptic_curve) is a curve defined by the equation**

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
This property is called *closure*.
For any two points $P$ and $P^\prime$ in the curve,
$P + P^\prime$ is also a point in the curve.

There are **two cases where the line doesn't intersect the curve in a third point**.
The first one is when $P = P^\prime$.

{{< figure src="ec_point_tangent.png" title="Adding a Point to Itself in an Elliptic Curve" >}}

Then we have a tangent line.
The other is when $P^\prime$ is the reflection of $P$ with respect to the $x$-axis.
Then we have a vertical line.

{{< figure src="ec_point_vertical.png" title="Adding a Point to Its Reflection in an Elliptic Curve" >}}

In both cases we define $P + P^\prime = \mathcal{O}$,
where $\mathcal{O}$ is the **point at infinity**.

### Point Addition Properties

Point addition satifies some properties:

- **Identity**: $P + \mathcal{O} = \mathcal{O} + P = P$
- **Commutativity**: $P + P^\prime = P^\prime + P$
- **Associavitiy**: $(P + P^\prime) + P^{\prime\prime} = P + (P^\prime + P^{\prime\prime})$
- **Invertibility**: $P + (-P) = \mathcal{O}$
- **Closure**: $P + P^\prime$ is a point in the curve

Hey this is very similar to addition of integers!
But using points instead of integers.

In fact we can define **multiplication** of a point $P$ by an integer $n$ as:

$$\underbrace{P + P + \cdots + P}_{n \text{ times}}$$

### Abelian Groups

The set of points in an elliptic curve with the point at infinity $\mathcal{O}$
along with a binary operation $+$ that satisfies the properties above is called an
[*Abelian group*](https://en.wikipedia.org/wiki/Abelian_group).
In honor of Niels Henrik Abel, an Abelain group is a **set with a binary operation that
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

Also there are two special elements in the field,
called the **additive identity** $-a$ and the **multiplicative identity** $a^{-1}$,
such that

$$a + (-a) = a,$$

and

$$a \times a^{-1} = a.$$

Note that this allows us to define **subtraction**

$$a - b = a + (-b),$$

and **division**

$$a \div b = a \times b^{-1}.$$

### Finite Fields

Now we are ready for finite fields.
A [*finite field*](https://en.wikipedia.org/wiki/Finite_field), also called a Galois field (in honor of Évariste Galois),
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

**The number of elements in a finite field is called the *order* of the field**.
The order of a finite field is **always a prime number $p$**.
The $\mathbb{Z}_4$ example above is a finite field of order 2.
To see that notice that $4$ is a composite number,

$$4 = 2 \times 2.$$

And we can write $\mathbb{Z}_4$ as

$$\mathbb{Z}_4 = 2 \times \mathbb{Z}_2.$$

This means that every element in $a \in \mathbb{Z}_4$ can be written as

$$a = 2 \times b,$$

where $b$ is an element in $\mathbb{Z}_2$.

Hence not every element of $\mathbb{Z}_4$ is unique and they are equivalent to the elements in $\mathbb{Z}_2$.

In general if $n$ is a composite number,
then $\mathbb{Z}_n$ is not a finite field.
However if $n = r \times s$ where $r$ and $s$ are prime numbers,
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

Multiplication in finite fields can be writen as multiple additions.
For example, in $\mathbb{Z}_3$,

$$2 \times 2 = 2 + 2 = 4 \mod 3 = 1.$$

Expontiation in finite fields can be writen as multiple multiplications.
For example, in $\mathbb{Z}_3$,

$$2^2 = 2 \times 2 = 4 \mod 3 = 1.$$

This is very trivial for any finite field.
However for division we are pretty much screwed.
It is really hard to find the multiplicative inverse of an element in a finite field.
For example, suppose that we have numbers $a,b$ in a very large finite field $\mathbb{Z}_n$,
such that

$$c = a \times b \mod n.$$

Then we can write division as

$$a = c \div b = c \times b^{-1} \mod n.$$

Now  we need to find $b^{-1}$.
This is called the [*discrete logarithm problem*](https://en.wikipedia.org/wiki/Discrete_logarithm).
Because we need to find the exponent $b^{-1}$ such that

$$b^{-1} = \log_b c \mod n.$$

Good luck my friend, no efficient method is known for computing them in general.
You can try brute force, but that's not efficient.

## Bringing it All Together

Now we are ready to talk about elliptic curve cryptography.
Elliptic curve cryptography is a public-key encryption technique based on elliptic curves.
It is used to create public and private keys for asymmetric cryptography.

## License

This post is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International][cc-by-nc-sa].

[![CC BY-SA 4.0][cc-by-nc-sa-image]][cc-by-nc-sa]

[cc-by-nc-sa]: http://creativecommons.org/licenses/by-nc-sa/4.0/
[cc-by-nc-sa-image]: https://licensebuttons.net/l/by-nc-sa/4.0/88x31.png

[^duality]: This is also why transaction malleability was a problem before Segwit in Bitcoin.
[^infinity]: If the line is vertical or tangent to the curve,
             then it intersects the curve in a point at infinity.
