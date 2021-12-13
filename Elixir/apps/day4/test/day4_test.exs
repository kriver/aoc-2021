defmodule Day4Test do
  use ExUnit.Case
  doctest Day4

  test "First win" do
    assert Day4.play()
           |> List.first()
           |> Map.get(:score) == 63424
  end

  test "Last win" do
    assert Day4.play()
           |> List.last()
           |> Map.get(:score) == 23541
  end
end
