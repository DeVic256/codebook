# [Largest Product in a Series](https://projecteuler.net/problem=8)

## Problem

To find the largest product of contiguous 13 numbers in the ___following series___. 

The largest 4-contiguous product given is $9 \times 9 \times 8 \times 9 = 5832$

73167176531330624919225119674426574742355349194934 <br>
96983520312774506326239578318016984801869478851843 <br>
85861560789112949495459501737958331952853208805511 <br>
12540698747158523863050715693290963295227443043557 <br>
66896648950445244523161731856403098711121722383113 <br>
62229893423380308135336276614282806444486645238749 <br>
30358907296290491560440772390713810515859307960866 <br>
70172427121883998797908792274921901699720888093776 <br>
65727333001053367881220235421809751254540594752243 <br>
52584907711670556013604839586446706324415722155397 <br>
53697817977846174064955149290862569321978468622482 <br>
83972241375657056057490261407972968652414535100474 <br>
821663704844031 ___9989___ 0008895243450658541227588666881 <br>
16427171479924442928230863465674813919123162824586 <br>
17866458359124566529476545682848912883142607690042 <br>
24219022671055626321111109370544217506941658960408 <br>
07198403850962455444362981230987879927244284909188 <br>
84580156166097919133875499200524063689912560717606 <br>
05886116467109405077541002256983155200055935729725 <br>
71636269561882670428252483600823257530420752963450

## Solution

The approach is to multiply all sequences of size $13$ in the input above.

This is an example of a 

|Sequence #|13-Sequence  |Product  |
|:--------:|:-----------:|:-------:|
|1         |7316717653133|5000940  |
|2         |3167176531330|0        |
|3         |1671765313306|0        |
|4         |6717653133062|0        |

### Oops 😱

As you can notice the last 3 sequences return zero since they have a $0$ included multiplying the whole sequence. 

Not only that but if I continue with this approach, I will be multiplying $13$ entire sequences for each $0$ which we can get rid of.

First, I take the zeroes out with a `split(0)`, that will give me a vector of sequences which do not contain any zeroes at all.

Solving that, next step is to get rid of sequences of less then $13$ numbers, due to these not being a requirement for me to process.

So I continue to `filter` by size all sequences and only then, process remaining sub-sequences to get the highest products as I would have done

in the beggining, but they are just in a different format. Notice I can't join them back together since the splitted sequences are no longer contiguous.

Here's how it would go:

```ruby
# This is for sure a seq with 13 or more numbers
def higher(seq)
  lw, up = 0, 12
  prod, higher = 1, 1

  while up < seq.length
      prod = seq[lw..up].product
      higher = prod if prod > higher
  end

  higher
end

# Main fn
def find_highest_product(input)
  input.split('0')
       .filter( |subseq| { subseq.length >= 13 } )
       .map(&:higher).max # Getting the max out of the largest subseq products
end
```