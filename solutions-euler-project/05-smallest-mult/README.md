# [Smallest Multiple](https://projecteuler.net/problem=5)

## Problem

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

## Solution

I have to find all a number that divides all numbers from 1 up to 20.

> Let $n$ be the number that divides 1 up to 20. 

> And $x_i$ the individual numbers such that $0 < x_i <= n$ 

What first came to my mind is that if $n | x_i$ then it should ok to check only the prime numbers in between.

> All the PN up to 20 are $\{ 2, 3, 5, 7, 11, 13, 17, 19 \}$

For my tests I used a hint from the same problem:

> $2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.

So I tried with $\{2, 3, 5, 7\}$. Which product is $210 $ and not $2520$.

If you start dividing $210$ for each $x_i$ you notice that only powers of prime numbers such that $p^y_i <= x_i$ are missing in the factors.

For instance:

$2*2*2*3*3*5*7$ returns indeed $2520$

So next step is to find those for $n = 20$ and listing all the factors:

$2*2*2*2*3*3*5*7*11*13*17*19$