require 'raylib'

def degree2rad(deg)
  (deg*Math::PI)/180
end

class Point
  attr_accessor :x, :y

  def initialize(**kwargs)
    @x, @y = kwargs[:x], kwargs[:y]
  end

  def ==(other)
    (@x == other.x) && (@y == other.y)
  end

  def -(other)
    if other.is_a? Integer
      Point.new x: @x - other, y: @y - other
    else
      Point.new x: @x - other.x, y: @y - other.y
    end
  end

  def +(other)
    if other.is_a? Integer
      Point.new x: @x + other, y: @y + other
    else
      Point.new x: @x + other.x, y: @y + other.y
    end
  end

  def *(other)
    Point.new x: @x*other, y: @y*other
  end

  def rotate(rad)
    zx = @x*Math.cos(rad) - @y*Math.sin(rad)
    zy = @x*Math.sin(rad) + @y*Math.cos(rad)
    Point.new(x: zx, y: zy)
  end
end

class Line
  attr_accessor :p1, :p2

  def initialize(**kwargs)
    @p1 = kwargs[:p1]
    @p2 = kwargs[:p2]
  end

  def length
    Math.sqrt(((@p1.x - @p2.x)**2) + ((@p1.y - @p2.y)**2))
  end

  def midpoint
    Point.new x: (@p1.x + @p2.x)/2, y: (@p1.y + @p2.y)/2
  end

  def *(other)
    Line.new p1: @p1, p2: @p2*other
  end

  def move2!(p0)
    aux = @p1 - p0
    @p1 -= aux
    @p2 -= aux
  end

  def move2(p0)
    aux = @p1 - p0
    p1 = @p1 - aux
    p2 = @p2 - aux
    Line.new p1: p1, p2: p2
  end

  def rotate_with(rad, p0)
    pa = (@p1 - p0).rotate(rad) + p0
    pb = (@p2 - p0).rotate(rad) + p0
    Line.new p1: pa, p2: pb
  end

  def rotate!(rad)  
    p0 = midpoint
    @p1 = (@p1 - p0).rotate(rad) + p0
    @p2 = (@p2 - p0).rotate(rad) + p0
  end

  def rotate(rad)
    p0 = midpoint
    pa = (@p1 - p0).rotate(rad) + p0
    pb = (@p2 - p0).rotate(rad) + p0
    Line.new p1: pa, p2: pb
  end
end

# Graphic part
include Raylib

shared_lib_path = Gem::Specification.find_by_name('raylib-bindings').full_gem_path + '/lib/'
Raylib.load_lib(shared_lib_path + 'libraylib.dll', raygui_libpath: shared_lib_path + 'raygui.dll', physac_libpath: shared_lib_path + 'physac.dll')

class Viz
  def initialize(**kwargs)
    @w, @h = kwargs[:screen].split('x').map{ |w| w.to_i }

    InitWindow(@w, @h, 'Lindenmeyers')
    SetTargetFPS(60)
  end

  def show
    init
    c = 0

    until WindowShouldClose()
      BeginDrawing()
      once && (c += 1) if c <= 0
      update 
      EndDrawing()
    end

    CloseWindow()
  end
end