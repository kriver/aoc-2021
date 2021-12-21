defmodule Day21Test do
  use ExUnit.Case
  doctest Day21

  @start {10, 9}

  test "Play D100" do
    assert Day21.play_d100(@start) == 918_081
  end

  test "Play D3 multi-verse" do
    assert Day21.play_d3(@start) == 158_631_174_219_251
  end
end
