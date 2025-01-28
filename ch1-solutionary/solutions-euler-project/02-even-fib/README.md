# [Even Fibonacci Numbers](https://projecteuler.net/problem=2)

The problem states: Find the sum of all fibonacci even-values below 4M.

[Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence) is defined as $F_n = F_{n - 1} + F_{n - 2}$ having $F_1 = 1, F_2 = 1$ as base cases.

```ruby
def fib(n)
  return 1 if n < 3
  fib(n - 1) + fib(n - 2)
end
```

Since fibonacci is a linear recurrent relation with constan coefficients you can get its [closed-formula](https://en.wikipedia.org/wiki/Fibonacci_sequence#Closed-form_expression):

$F_n = \frac{\phi^n - \psi^2}{\sqrt{5}}$

That would turn our target function $\Omicron (1)$

```ruby
def fib(n)
  num = pow(phi, n) - pow(psi, n) 
  num / sqrt 5
end
```

By evaluating some values you can see which arguments generate even values

$F_3 => 2$

$F_4 => 3$

$F_5 => 5$

$F_6 => 8$

$F_7 => 13$

$F_8 => 21$

$F_9 => 38$

$F_{10} => 55$

$F_{11} => 89$

$F_{12} => 144$

As we can see the values for that $F = 2n$ are $3, 6, 9, 12,$ ... All multiples of $3$.

Therefore we can assume that $F_{3n} = 2m$ => $\forall n, m \epsilon N$

Then the the result would be $\Sigma_{i=1}^n F_{3i}$

```ruby
def sum_even_fib(n)
    (0..n).map { |i| fib(3*i) }.sum
end
```

And now find the values under 4M