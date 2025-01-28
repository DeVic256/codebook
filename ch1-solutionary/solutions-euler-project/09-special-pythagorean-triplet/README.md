# [Special Pythagorean Triplet](https://projecteuler.net/problem=9)

## Problem

A Pythagorean triplet is a set of three natural numbers $a$, $b$, $c$ for which, $a < b < c$
For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.

There exists exactly one Pythagorean triplet for which $a+b+c=1000$.

Find the product $abc$.

## Solution

I have to ___find___ a pythagorean triplet for which $a+b+c=1000$ which could potentially be $\Omicron(abc)$.

So if the triplet I'm looking for is $(24, 15, 34)$, to find it by guessing one by one it could take $24 \times 15 \times 34 = 12240$ guesses minus the ones below $(3, 4, 5)$ so totally $12180$.

So I'm going to try guess from a higher triplet so I save some steps. With that in mind I will find the highest triplet to begin my search. So far we have that $3+4+5=12$ and we need numbers who's addition is closer to $1000$ before start searching; so let's get triplets derived from $(3,4,5)$ and see who's addition is closer not to $1000$, but a lower factor of $1000$. By doing this we can derive the triplet if we find one before.

Factors of $1000$ are:

| Divisors| Factors  |
|:-------:|:--------:|
|1000     |2         |
|500      |2         |
|250      |2         |
|125      |5         |
|25       |5         |
|5        |5         |

And $25 \times 40 = 1000$ since $5 \times 2^3 = 40$

So starting with triplets who's addition is closer to $25$ and a bottom start with $(3, 4, 5)$ I will multiply this triplet by squares.

$ 3^2 + 4^2 = 5^2 (\times 2^2) $ <br>
$ 6^2 + 8^2 = 10^2 => 6+8+10=24$

We can start with this one $(6, 8, 10)$ and do the same for each factor. In theory this ___special triplet___ in these places. Such that, $(3d, 4d, 5d) < p < (3d', 4d', 5d')$ where $p$ is a factor of $1000$ and $d$ is a squared number that will transform the triplet into another derived aand $d'$ is the next squared number.

Let's try with $25$. So the bounding triplets are $(6, 8, 10)$ and $(9, 12, 15)$:

$ 3^2 + 4^2 = 5^2 (\times 3^2) $ <br>
$ 9^2 + 12^2 = 15^2 => 9 + 12 + 15 = 36$

```ruby
for i in 6..9
  for j in 8..12
    for k in 10..15
      return [i, j, k] if [i, j, k].sum % 25 == 0 && triplet?(i, j, k)
return nil
```

This one doesn't yield any triplets so now let's do this generally for each factor.

```ruby
def find_triplet(t1, t2, f)
  for i in t1.a..t2.a
    for j in t1.b..t2.b
      for k in t1.c..t2.c
        return [i, j, k] if [i, j, k].sum % f == 0 && triplet?(i, j, k)
  return nil
end

def find_bound_triplets(f)
  lower = Math.sqrt(find_lower_square(f))
  higher= Math.sqrt(find_higher_square(f))

  lbound = [3,4,5].map(|t| t*lower)
  hbound = [3,4,5].map(|t| t*higher)

  return [lbound, hbound]
end

def main
  [25, 40, 50, 125, 250, 500, 1000].each do |factor|
    lbound, hbound = find_bound_triplets(factor)
    triplet = find_triplet(lbound, hbound, factor)
    break if !triplet.nil?
  end

  if triplet.sum != 1000
    factor = 1000/triplet.sum
    triplet.map(|term| term*factor)
   else
    triplet
  end 
end
```