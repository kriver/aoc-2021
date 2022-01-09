defmodule Day7Test do
  use ExUnit.Case
  doctest Day7

  test "Constant fuel rate" do
    assert Day7.run(&Day7.median/1, &Day7.constant_fuel/2) == 343468
  end

  test "Incremental fuel rate" do
    assert Day7.run(&Day7.average/1, &Day7.incremental_fuel/2) == 96086265
  end
end
