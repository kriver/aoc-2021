defmodule Day13 do
  @moduledoc """
  Documentation for `Day13`.
  """

  @dot_re ~r/^(\d+),(\d+)$/
  @fold_re ~r/^([xy])=(\d+)$/

  def parse_line("", results), do: results

  def parse_line("fold along " <> line, {dots, folds}) do
    [dir | [pos | _]] = Regex.run(@fold_re, line, capture: :all_but_first)
    {dots, folds ++ [{dir, String.to_integer(pos)}]}
  end

  def parse_line(line, {dots, folds}) do
    [x | [y | _]] = Regex.run(@dot_re, line, capture: :all_but_first)
    {MapSet.put(dots, {String.to_integer(x), String.to_integer(y)}), folds}
  end

  def input() do
    File.read!("data/day13.txt")
    |> String.split("\n")
    |> Enum.reduce({MapSet.new(), []}, &parse_line/2)
  end

  def fold(dots, {"x", fold_at}) do
    MapSet.new(
      dots
      |> MapSet.to_list()
      |> Enum.map(fn {x, y} -> {(if x > fold_at, do: 2 * fold_at - x, else: x), y} end)
    )
  end

  def fold(dots, {"y", fold_at}) do
    MapSet.new(
      dots
      |> MapSet.to_list()
      |> Enum.map(fn {x, y} -> {x, (if y > fold_at, do: 2 * fold_at - y, else: y)} end)
    )
  end

  def fold_first() do
    {dots, instructions} = input()
    instructions
    |> Enum.slice(0..0)
    |> Enum.reduce(dots, fn instr, acc -> fold(acc, instr) end)
    |> Enum.count()
  end

  def fold_all() do
    {dots, instructions} = input()
    instructions
    |> Enum.reduce(dots, fn instr, acc -> fold(acc, instr) end)
    |> Enum.count()
  end
end
