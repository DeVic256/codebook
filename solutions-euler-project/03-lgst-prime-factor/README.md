# Largest Prime Factor

## The problem:

> What is the largest prime factor of the number $600851475143$?

## The Solution

To solve this you need to get all the **prime factors** between $2$ and $n$. The task is slow and even more for the given number. However, we can do some tricks to get the time shortened by a lot.

First step is to know if a number is prime:

```ruby
def prime?(n)
  for i in 2..n.sqrt()
    if n % i == 0 return false
  end

  return true
end
```

The code checks for $n$'s divisors but only up to $\sqrt{n}$ there's a bit more efficient way to do this by using a [Scieve of Erathosteles](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) or [Trial Division](https://en.wikipedia.org/wiki/Trial_division). This is important because it'll take ***prime?(n)*** from $\Omicron(n)$ to $\Omicron(\sqrt{n})$.

Final step is to check for prime factors from $2$ up to $\sqrt{n}$ and evaluate for the big number

```ruby
def factor?(n, k)
  n % k == 0
end
```

```ruby
def largest_prime(n)
  lrgst_prime = 2

  for i in 2..n.sqrt()
    lrgst_prime = i if prime?(n) && factor?(n, i)
  end

  lrgst_prime 
end
```

```ruby
def main
  puts largest_prime(600851475143)
end
```
