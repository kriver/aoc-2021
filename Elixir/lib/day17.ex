defmodule Day17 do
  @moduledoc """
  Documentation for `Day17`.
  """

  @x1   60
  @y1 -171
  @x2   94
  @y2 -136

  require Math

  def vx_for(x) do
    trunc(ceil((-1 + Math.sqrt(1 + 8 * x)) / 2))
  end

  def in_area({x, y}), do: @x1 <= x and x <= @x2 and @y1 <= y and y <= @y2

  def overshot({x, y}), do: x > max(@x1, @x2) or y < min(@y1, @y2)

  def move({x, y}, {vx, vy}), do: {{x + vx, y + vy}, {max(0, vx - 1), vy - 1}}

  def move(pos, velocity, highest) do
    cond do
      in_area(pos) -> highest
      overshot(pos) -> -1
      true -> {{x, y}, nv} = move(pos, velocity)
              move({x, y}, nv, max(y, highest))
    end
  end

  def move(velocity) do
    move({0, 0}, velocity, 0)
  end

  def find_highest({_vx, vy}, highest) when vy > abs(@y1) + 1, do: highest
  def find_highest({vx, vy}, highest) do
    find_highest({vx, vy + 1}, max(highest, move({vx, vy})))
  end
  def find_highest() do
    find_highest({vx_for(@x1), -(@y2 + 1)}, 0)
  end

  def find_all() do
    (@y1 - 1)..(-@y1 + 1)
    |> Enum.flat_map(
         fn vy ->
           vx_for(@x1)..(@x2 + 1)
           |> Enum.map(fn vx -> move({vx, vy}) end)
         end
       )
    |> Enum.filter(fn v -> v >= 0 end)
  end
end
