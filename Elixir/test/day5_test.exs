defmodule Day5Test do
  use ExUnit.Case
  doctest Day5

  test "Overlaps non-diagonal" do
    assert Day5.run(false) == 4873
  end

  test "Overlaps allow diagonal" do
    assert Day5.run(true) == 19472
  end
end
