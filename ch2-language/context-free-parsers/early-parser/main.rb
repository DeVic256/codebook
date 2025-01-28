require './tokenizer'
require './parser'

t = Tokenizer.new 'p ^ (q or r) -> t'
tkns = t.tokenize

p "#{tkns}"

g = Grammar.new(start_name: 'statment')

g.add_rule Rule.new(name: 'statement', symbols: ["(", "statement", ")"])
g.add_rule Rule.new(name: 'statement', symbols: ["statement", "^", "term"])
g.add_rule Rule.new(name: 'statement', symbols: ["statement", "or", "term"])
g.add_rule Rule.new(name: 'statement', symbols: ["statement", "->", "term"])
g.add_rule Rule.new(name: 'statement', symbols: ["term"])
g.add_rule Rule.new(name: 'term',      symbols: ["!", "term"])
g.add_rule Rule.new(name: 'term',      symbols: ["variable"])
g.add_rule Rule.new(name: 'variable',  symbols: ["p", "q", "r", "s", "t", "u"])

ei_parser = EarlyParser.new(grammar: g)
ei_parser.build_items(input: tkns)

print ei_parser