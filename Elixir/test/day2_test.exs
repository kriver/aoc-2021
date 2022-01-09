defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "Position product (no aim)" do
    assert Day2.run_no_aim() == 1561344
  end

  test "Position product (with aim)" do
    assert Day2.run_with_aim() == 1848454425
  end
end
