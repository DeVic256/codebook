# [Largest Palindrome Product](https://projecteuler.net/problem=4)

## Problem

> Find the largest palindrome made from the product of two $3$-digit numbers

## Solution

Let $n$ be a palindrome number product of two $3$-digit numbers, and $k, j$ these two numbers, then:

$j = 100a + 10b + c$

$k = 100d + 10e + f$

$n = jk = (100a + 10b + c)(100d + 10e + f)$

Expanding:

$n = 10000ad + 1000ae + 100af + 1000bd + 100be +10bf + 100cd + 10ce + cf$

Common factors:

$n = 10000ad + 1000(ae + bd) + 100(af + be + cd) + 10(bf + ce) + cf$

Since $n$ is palindrome we necesarely have $ad = cf, ae + bd = bf + ce$, but since $ad > 9 | a = 2, d = 5$ we can't use the full expression above to find our number. Rather, I'll use the first three places, and renaming the places we have:

$n = 100000cf + 10000(bf + ce) + 1000(af + be + cd) + 100(af + be + cd) + 10(bf + ce) + cf$

Now that we have a more accurate representation let's simplify:

$n = 100001cf + 10010(bf + ce) + 1100(af + be + cd)$

```ruby
def palindrome?(n)
  n == 100001*place(n, 1) + 10010*place(n, 2) + 1100*place(n, 3)
end
```

Now we check all $ij$ where $200 < i < 999$ and $500 < j < 999$

```ruby
def largest_palindrome_factor()
  for i in 999..200
    for j in 999..500
        return i*j if palindrome?(i*j)
    end
  end
end
```