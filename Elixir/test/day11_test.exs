defmodule Day11Test do
  use ExUnit.Case
  doctest Day11

  test "Number of flashes" do
    assert Day11.flash_count(100) == 1652
  end

  test "All flash cycle" do
    assert Day11.all_flash() == 220
  end
end
