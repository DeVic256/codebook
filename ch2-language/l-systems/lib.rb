class Grammar
  attr_accessor :axiom, :rules

  def initialize(**kwargs)
    @axiom = kwargs[:axiom]
    @rules = {}

    # Static rules that never change
    rule '[', '['
    rule ']', ']'
    rule '+', '+'
    rule '-', '-'
  end

  def rule(init, gen)
    @rules[init] = gen
  end
end

class Lindenmayer
  attr_reader :g, :delta

  def initialize
    model
    loop { self.next; @n-=1; break if @n <= 0 } if @n > 0
  end

  def init
    model
  end

  def grammar_axiom(a)
    @g = Grammar.new(axiom: a)
  end

  def grammar_rule(ri, ru)
    @g.rule(ri, ru)
  end

  def iterations(c)
    @n = c
  end

  def angle(d)
    @delta = d
  end

  def axiom
    @g.axiom
  end

  # Reimplement next to work with tokens if needed
  # For more complex examples it may need CFG parsing
  def next
    @g.axiom = @g.axiom.chars.map { |c| @g.rules[c] }.join
  end
end
