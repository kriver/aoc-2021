defmodule Day12Test do
  use ExUnit.Case
  doctest Day12

  test "Paths small only once" do
    assert Day12.paths_once() == 5076
  end

  test "Paths small only once, one twice" do
    assert Day12.paths_one_twice() == 145643
  end
end
