# [Sum Square Difference](https://projecteuler.net/problem=6)

## Problem

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

## Solution

Both summations are [common progressions](https://en.wikipedia.org/wiki/Summation#Powers_and_logarithm_of_arithmetic_progressions) with closed formula identities. Hence:

$\Sigma^n_{i=1}{i} = \frac{n(n+1)}{2} = M$

and

$\Sigma^n_{i=1}{i^2} = \frac{n(n+1)(2n+1)}{6} = N$

so we are asked to find $f(n) = M^2-N$ where $n$ is $100$ which is a $\Omicron (1)$ operation and doing some basic algebra we get:

$M^2 - N = \frac{n(3n+2)(n^2 - 1)}{12}$ and evaluate $f(100)$

