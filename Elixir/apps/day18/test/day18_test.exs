defmodule Day18Test do
  use ExUnit.Case
  doctest Day18

  test "To tuple" do
    assert Day18.to_tuple("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]") == {
             {{9, {3, 8}}, {{0, 9}, 6}},
             {{{3, 7}, {4, 9}}, 3}
           }
  end

  test "Split" do
    assert Day18.split({11, 12}) == {{5, 6}, 12}
    assert Day18.split({1, 11}) == {1, {5, 6}}
    assert Day18.split({1, {11, 12}}) == {1, {{5, 6}, 12}}
    assert Day18.split({{11, 12}, 1}) == {{{5, 6}, 12}, 1}
    assert Day18.split({{{{0, 7}, 4}, {15, {0, 13}}}, {1, 1}}) == {{{{0, 7}, 4}, {{7, 8}, {0, 13}}}, {1, 1}}
    assert Day18.split({{{{0, 7}, 4}, {{7, 8}, {0, 13}}}, {1, 1}}) == {{{{0, 7}, 4}, {{7, 8}, {0, {6, 7}}}}, {1, 1}}
  end

  test "Explode" do
    assert Day18.explode({{{{{9, 8}, 1}, 2}, 3}, 4}) == {{{{0, 9}, 2}, 3}, 4}
    assert Day18.explode({7, {6, {5, {4, {3, 2}}}}}) == {7, {6, {5, {7, 0}}}}
    assert Day18.explode({{6, {5, {4, {3, 2}}}}, 1}) == {{6, {5, {7, 0}}}, 3}
    assert Day18.explode({{3, {2, {1, {7, 3}}}}, {6, {5, {4, {3, 2}}}}}) == {{3, {2, {8, 0}}}, {9, {5, {4, {3, 2}}}}}
    assert Day18.explode({{3, {2, {8, 0}}}, {9, {5, {4, {3, 2}}}}}) == {{3, {2, {8, 0}}}, {9, {5, {7, 0}}}}
  end

  test "Reduce" do
    assert Day18.reduce({{{{{4, 3}, 4}, 4}, {7, {{8, 4}, 9}}}, {1, 1}}) == {{{{0, 7}, 4}, {{7, 8}, {6, 0}}}, {8, 1}}
  end

  test "Add + reduce" do
    lhs = Day18.to_tuple([[[[4, 3], 4], 4], [7, [[8, 4], 9]]])
    rhs = Day18.to_tuple([1, 1])
    sum = Day18.to_tuple([[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
  end

  test "Add + reduce chain" do
    lhs = Day18.to_tuple([[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]])
    rhs = Day18.to_tuple([7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]])
    sum = Day18.to_tuple([[[[4, 0], [5, 4]], [[7, 7], [6, 0]]], [[8, [7, 7]], [[7, 9], [5, 0]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([[2, [[0, 8], [3, 4]]], [[[6, 7], 1], [7, [1, 6]]]])
    sum = Day18.to_tuple([[[[6, 7], [6, 7]], [[7, 7], [0, 7]]], [[[8, 7], [7, 7]], [[8, 8], [8, 0]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([[[[2, 4], 7], [6, [0, 5]]], [[[6, 8], [2, 8]], [[2, 1], [4, 5]]]])
    sum = Day18.to_tuple([[[[7, 0], [7, 7]], [[7, 7], [7, 8]]], [[[7, 7], [8, 8]], [[7, 7], [8, 7]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([7, [5, [[3, 8], [1, 4]]]])
    sum = Day18.to_tuple([[[[7, 7], [7, 8]], [[9, 5], [8, 7]]], [[[6, 8], [0, 8]], [[9, 9], [9, 0]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([[2, [2, 2]], [8, [8, 1]]])
    sum = Day18.to_tuple([[[[6, 6], [6, 6]], [[6, 0], [6, 7]]], [[[7, 7], [8, 9]], [8, [8, 1]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([2, 9])
    sum = Day18.to_tuple([[[[6, 6], [7, 7]], [[0, 7], [7, 7]]], [[[5, 5], [5, 6]], 9]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([1, [[[9, 3], 9], [[9, 0], [0, 7]]]])
    sum = Day18.to_tuple([[[[7, 8], [6, 7]], [[6, 8], [0, 8]]], [[[7, 7], [5, 0]], [[5, 5], [5, 6]]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([[[5, [7, 4]], 7], 1])
    sum = Day18.to_tuple([[[[7, 7], [7, 7]], [[8, 7], [8, 7]]], [[[7, 0], [7, 7]], 9]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
    lhs = sum
    rhs = Day18.to_tuple([[[[4, 2], 2], 6], [8, 7]])
    sum = Day18.to_tuple([[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]])
    assert Day18.reduce(Day18.add(lhs, rhs)) == sum
  end

  test "Magnitude" do
    assert Day18.magnitude({{{{6, 6}, {7, 6}}, {{7, 7}, {7, 0}}}, {{{7, 7}, {7, 7}}, {{7, 8}, {9, 9}}}}) == 4140
  end

  test "Homework" do
    assert Day18.homework() == 2907
  end

  tes "Backside" do
    assert Day18.backside() == 4690
  end
end
