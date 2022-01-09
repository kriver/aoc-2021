defmodule Day17Test do
  use ExUnit.Case
  doctest Day17

  test "Highest point" do
    assert Day17.find_highest() == 14535
  end

  test "All valid velocities" do
    assert length(Day17.find_all()) == 2270
  end
end
