# [Multiples of 3 or 5](https://projecteuler.net/problem=1)

The problem states Find the sum of all the multiples of 3 or 5 below 1000.

## Solution

### The most obvious one

> This one consists in looping from 1 upto 1000 and checking all numbers for multiplicity.

```ruby
def mult
  result = 0

  for i in (1..1000)
    result += i if i % 3 == 0 || i % 5 == 0
  end

  return result
end
```

### The most efficient one

This one consists in the following identity: $\Sigma_{i=1}^n n = \frac{n(n+1)}{2}$

```ruby
def sum_to_n(n)
  return n * (n + 1) / 2
end

def mult
  return [3, 5, 15].map { |n| n*sum_to_n(1000 / n) }.sum
end
```

### Proof 

> To prove that $\Sigma_{i=1}^n n = \frac{n(n+1)}{2}$ we need to expand the whole sum like this
<br>

$S = 1 + 2 + ... + (n-1) + n$
<br>

> By reversing the order it looks like this

$S = n + (n-1) + ... + 2 + 1$
<br>

> Adding both together we get
<br>

$2S = (n+1) + (n+1) + ... + (n+1) + (n+1)$
<br>

> Simplifying and passing the 2 to the right-hand side we get
<br> <br>
$S = \frac{n(n+1)}{2}$
<br> <br>