defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "Window size of 1" do
    assert Day1.run(1) == 1722
  end

  test "Window size of 3" do
    assert Day1.run(3) == 1748
  end
end
