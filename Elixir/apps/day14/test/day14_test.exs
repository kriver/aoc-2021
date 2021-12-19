defmodule Day14Test do
  use ExUnit.Case
  doctest Day14

  test "Quantity difference after 10 generations" do
    assert Day14.evolve(10)
           |> Day14.score() == 2657
  end

  test "Quantity difference after 40 generations" do
    assert Day14.evolve(40)
           |> Day14.score() == 2911561572630
  end
end
