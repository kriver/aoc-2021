defmodule Day20Test do
  use ExUnit.Case
  doctest Day20

  test "Lit pixels after 2 generations" do
    assert Day20.lit_pixels(Day20.evolve(2)) == 5464
  end

  test "Lit pixels after 50 generations" do
    assert Day20.lit_pixels(Day20.evolve(50)) == 19228
  end
end
