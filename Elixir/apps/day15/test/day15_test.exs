defmodule Day15Test do
  use ExUnit.Case
  doctest Day15

  test "Lowest risk path" do
    assert Day15.find_path() == 373
  end
end
