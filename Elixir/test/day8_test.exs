defmodule Day8Test do
  use ExUnit.Case
  doctest Day8

  test "1, 4, 7 and 8's" do
    assert Day8.count_1478() == 310
  end
end
