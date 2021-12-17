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

  def visualise(dots) do
    max_x = dots
            |> Enum.map(fn {x, _y} -> x end)
            |> Enum.max()
    max_y = dots
            |> Enum.map(fn {_x, y} -> y end)
            |> Enum.max()
    Enum.reduce(
      0..max_y,
      [],
      fn y, acc ->
        [
          Enum.reduce(
            0..max_x,
            [],
            fn x, acc2 ->
              [(if MapSet.member?(dots, {max_x - x, max_y - y}), do: "#", else: ".") | acc2]
            end
          ) | acc
        ]
      end
    )
    |> Enum.map(fn l -> Enum.join(l) end)
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

  def visualise() do
    {dots, instructions} = input()
    instructions
    |> Enum.reduce(dots, fn instr, acc -> fold(acc, instr) end)
    |> visualise()
  end
end
