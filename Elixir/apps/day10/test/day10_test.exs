defmodule Day10Test do
  use ExUnit.Case
  doctest Day10

  test "Corruption score" do
    assert Day10.part1() == 318099
  end

  test "Incomplete score" do
    assert Day10.part2() == 2389738699
  end
end
