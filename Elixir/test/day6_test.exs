defmodule Day6Test do
  use ExUnit.Case
  doctest Day6

  test "Fish after 80 days" do
    assert Day6.run(80) == 345387
  end

  test "Fish after 256 days" do
    assert Day6.run(256) == 1574445493136
  end
end
