defmodule Day9Test do
  use ExUnit.Case
  doctest Day9

  test "Low points" do
    assert Day9.low_point_risks() == 480
  end
end
