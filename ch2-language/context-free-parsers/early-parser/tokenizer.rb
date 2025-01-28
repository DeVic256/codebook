class Tokenizer
    attr_accessor :tokens

  def initialize(exp)
     @expression = exp 
     @aux = ""
     @tokens = []
  end

  def tokenize
    @expression = "#{@expression}\n"

    pcount = 0

    for c in @expression.chars
      raise "Missmatched parenthesis" if pcount < 0

      case c
      when '(' 
        pcount += 1
        publish
        @aux = '('
        publish
      when ')'
        pcount -= 1
        publish
        @aux = ')'
        publish
      when ' '
        publish
      when "\n"
        raise "Missmatched parenthesis" if pcount > 0
        publish
      else
        @aux += c
      end
    end

    @tokens
  end

  def to_s
    @tokens.to_s
  end

  private
  def publish
    @tokens.push @aux unless @aux.empty?
    reset
  end

  def reset
    @aux = ""
  end
end