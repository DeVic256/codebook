require './grammar'

#TODO: FINISH
class EarlyParser
  def initialize(**kwargs)
    @g = kwargs[:grammar]
    @s = [] #List of StateSets Vec<Set<Ei>>
    @cur_ss = 0
  end

  # TODO: Make this grammar agnostic
  def terminal?(sym)
    ['q','r','t','u','p','->','or', '^'].member?(sym)
  end

  def build_items(**kwargs)
    startup

    input = kwargs[:input]

    loop do
      predict(@cur_ss, input)
      scan(@cur_ss, input)
      complete(@cur_ss, input)

      print self
      sleep(3)

      break if @cur_ss >= input.length
    end
  end

  def add_to_state_set(**kwargs)
    ss = kwargs[:state_set]
    ei = kwargs[:ei]
    @s.push(Set.new) if ss >= @s.length
    @s[ss].add ei
  end

  def startup
    s = Set.new

    # TODO: Make it lang agnostic
    @g.rules.each_with_index do |ru, i|
      s.add EarlyItem.new(grammar_index: i, fat_dot: 0, start: 0) if ru.name == 'statement'
    end

    @s.push s
  end

  # Symbol @ right of the dot is non-terminal
  # Add to corresponding rules to the current state set
  def predict(cur_ss, input)
    aux_set = Set.new

    @s[cur_ss].each do |ei|
      rule = @g.find_rule(by: :ei, value: ei)

      sym = rule[ei.fat_dot]

      if !terminal?(sym)
        rules = @g.find_all_rules { |ru| ru.name == sym }
        aux_set.merge(rules.map { |ru| @g.rule_to_ei(start: cur_ss, rule: ru) })
      end
    end

    @s[cur_ss].merge(aux_set)
  end

  # The symbol @ right of the dot is terminal
  # We check if the input matches this symbol.
  # If it does, we add this item (advanced one step) to the next state set.
  def scan(cur_ss, input)
    @s[cur_ss].each do |ei|
      rule = @g.find_rule(by: :ei, value: ei) 
      sym = rule[ei.fat_dot]

      if terminal?(sym) && input[cur_ss] == sym
        add_to_state_set(state_set: cur_ss + 1, ei: ei) 
        next_state
      end
    end
  end

  # There is nothing at the right of the fat dot.
  # This means we have a successful partial parse.
  # We look for the parent items, and add them (advanced 1-step) to this state set.
  def complete(cur_ss, input)
  end

  def to_s
    i, r = 0, "Grammar\n#{@g}\n\n"

    while i < @s.length
      r += "State Set #{i}\n"
      ss = @s[i] 
      r += ss.map { |ei| ei.ref_from(grammar: @g); ei.to_s }.join("\n")
      i += 1
      r += "\n\n"
    end

    r
  end
  
  private
  def next_state
    @cur_ss += 1
  end
end