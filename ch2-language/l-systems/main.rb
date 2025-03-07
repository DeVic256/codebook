require './lib'
require './viz'
#require './systems' # Examples

class Lindenmayer
  def model
    iterations 3
    angle 60

    grammar_axiom 'F++F++F'
    grammar_rule 'F', 'FF++F--F++FF'
  end
end

#TODO: Refactor
class TurtleViz < Viz
  def init
    @p1 = Point.new(x: @w/2, y: @h/4)
    @p2 = Point.new(x: @w/2, y: (@h/4) + 20)
    @line0 = Line.new(p1: @p1, p2: @p2)

    @l = Lindenmayer.new
  end

  def F
    draw
    @line0.move2!(@line0.p2)
  end

  def pF
    @line0 = @line0.rotate_with(degree2rad(@l.delta), @line0.p1)
  end

  def mF
    @line0 = @line0.rotate_with(-degree2rad(@l.delta), @line0.p1)
  end

  def draw
    DrawLine(@line0.p1.x, @line0.p1.y, @line0.p2.x, @line0.p2.y, Raylib::RED) 
  end

  def update; end

  def once 
    DrawLine(@line0.p1.x, @line0.p1.y, @line0.p2.x, @line0.p2.y, Raylib::BLUE) 
    
    @l.axiom.chars.each do |a|
      case a
      when 'F'
        F()
      when '+'
        pF()
      when '-'
        mF()
      else
      end
    end
  end
end

TurtleViz.new(screen: '1200x900').show