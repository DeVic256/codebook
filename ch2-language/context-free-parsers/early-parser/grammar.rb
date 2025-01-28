class EarlyItem
  attr_reader :grammar_index, :fat_dot, :start, :gref

  def initialize(**kwargs)
    @grammar_index = kwargs[:grammar_index]
    @fat_dot = kwargs[:fat_dot]
    @start = kwargs[:start]
  end

  def hash
    "#{@grammar_index}-#{fat_dot}-#{start}".hash
  end

  def eql?(other)
    one = @grammar_index == other.grammar_index
    two = @fat_dot == other.fat_dot
    three = @start == other.start
    return (one && two && three)
  end

  def ref_from(**kwargs)
    @gref = kwargs[:grammar]
  end

  def to_s
    rule = @gref.find_rule(by: :ei, value: self)
    r = "#{rule.name} -> "
    rule.symbols.each_with_index do |sym, i|
      r += '*' if i == @fat_dot
      r += " #{sym}"
    end
    "#{r} (#{@start})"
  end
end

class Rule
  attr_reader :name, :symbols

  def initialize(**kwargs)
    @name = kwargs[:name]
    @symbols = kwargs[:symbols]
  end
  
  def [](index)
    @symbols[index]
  end

  def to_s
    "#{name} => #{symbols.join(' ')}"
  end
end

class Grammar
  attr_reader :rules
  attr_accessor :start_name

  def initialize(**kwargs)
    @start_rule_name = kwargs[:start_name]
    @rules = []
  end

  def [](index)
    @rules[index]
  end

  def add_rule(rule)
    @rules.push rule if rule.is_a?(Rule)
  end

  def find_all_rules(&where)
    @rules.filter(&where)
  end

  def find_rule(**kwargs)
    i, go, by, value = 0, nil, kwargs[:by], kwargs[:value]

    while i < @rules.length
      rule = @rules[i]
        
      case by
      when :name
        return rule if rule.name == value
      when :ei
        return rule if kwargs[:value].grammar_index == i
      when :terminal
        raise StandardError.new 'Find by Terminal not impld'
      else
        raise StandardError.new "Find by #{by} not impld"
      end

      i += 1
    end

    go
  end

  def rule_to_ei(**kwargs)
    i = 0
    ru = kwargs[:rule]

    while i < @rules.length do
      ei = EarlyItem.new(grammar_index: i, fat_dot: 0, start: (kwargs[:start] || 0))
      rui = @rules[i]
      return ei if (ru.name == rui.name && ru.symbols.join == rui.symbols.join )
      i += 1
    end

    return nil
  end

  def to_s
    @rules.map { |r| r.to_s }.join("\n")
  end
end