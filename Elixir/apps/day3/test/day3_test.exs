defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  test "Power consumption" do
    assert Day3.power_consumption() == 4118544
  end

  test "Life support rating" do
    assert Day3.life_support() == 3832770
  end
end
