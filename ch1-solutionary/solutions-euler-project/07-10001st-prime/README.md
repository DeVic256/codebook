# 10001st Prime

## Problem

> Find the $10001$st prime

## Solution

For this one I use a lookup table of primes, starting by 

```ruby
primes = [2, 3, 5, 7, 11, 13]
```

and check for primality against the lookup array, while having two counters. One for all numbers indefinitely and another to count primes.

```ruby
i, pcount = 14, 6

while pcount < 10001
  pcount += 1 && push i if prime? i
  i += 1
end
```

and finally return the last item in the list.