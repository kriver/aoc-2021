defmodule Day16Test do
  use ExUnit.Case
  doctest Day16

  def to_bits(s) do
    String.split(s, "", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  test "Parse literal" do
    assert Day16.to_pkt(to_bits("110100101111111000101000")) == {6, 4, 2021}
  end
  test "Parse length type" do
    assert Day16.to_pkt(to_bits("00111000000000000110111101000101001010010001001000000000")) == {
             1,
             6,
             [{6, 4, 10}, {2, 4, 20}]
           }
  end
  test "Parse count type" do
    assert Day16.to_pkt(to_bits("11101110000000001101010000001100100000100011000001100000")) == {
             7,
             3,
             [{2, 4, 1}, {4, 4, 2}, {1, 4, 3}]
           }
  end

  test "Version sum" do
    assert Day16.version() == 879
  end

  test "Evaluate" do
    assert Day16.evaluate() == 539051801941
  end
end
