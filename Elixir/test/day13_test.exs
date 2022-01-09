defmodule Day13Test do
  use ExUnit.Case
  doctest Day13

  test "Dots after one fold" do
    assert Day13.fold_first() == 669
  end

  test "Dots after all folds" do
    assert Day13.fold_all() == 90
  end
end
