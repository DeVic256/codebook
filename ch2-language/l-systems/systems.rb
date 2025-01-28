require './lib'

class QuadraticKochIsland < Lindenmayer
  def init
    iterations 3
    angle 90

    grammar_axiom 'F+F+F+F'
    grammar_rule 'F', 'F+F-F-FF+F+F-F'
  end
end

class SnowflakeCurveMod < Lindenmayer
  def init
    iterations 4
    angle 90

    grammar_axiom '+F'
    grammar_rule 'F', 'F-F+F+F-F'
  end
end
