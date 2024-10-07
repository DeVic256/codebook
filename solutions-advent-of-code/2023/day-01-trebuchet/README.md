# [Day 1: Trebuchet?](https://adventofcode.com/2023/day/1)

## Solution

Simply find the first occurrence of a digit in the word, then reverse the word and do the same. Then concatenate both and convert to integer. Do that for all words and sum the values. The choosing of reverse is also efficient since reverse iterator should be implemented to iterate backwards and not to return a new array with the reversed list. Plus, worst case is $\Omicron (n)$ and best case $\Omicron (1)$ 

```ruby
def num_extraction(word)
    fn = word.match /\d/
    ln = word.reverse().match /\d/

    fn + ln
end

def trebuchet(fname)
  words = File.readlines(fname)

  words.map{ |word| num_extraction(word).to_i }.sum
end
```

## Second Part

For the second part I just get the indexes where each substr digit word appears and get the first and last indexes. Once having that, sort the resulting list so I get the real first and last digits.