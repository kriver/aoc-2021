defmodule Day9Test do
  use ExUnit.Case
  doctest Day9

  test "Low points" do
    assert Day9.low_point_risks() == 480
  end

  test "Largest basins" do
    assert Day9.product_largest_basin_sizes(3) == 1045660
  end
end
