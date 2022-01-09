defmodule Day10 do
  @moduledoc """
  Documentation for `Day10`.
  """

  @pairs %{"(" => ")", "[" => "]", "{" => "}", "<" => ">"}
  @corrupt %{")" => 3, "]" => 57, "}" => 1197, ">" => 25137}
  @incomplete %{")" => 1, "]" => 2, "}" => 3, ">" => 4}

  def eat([], expect) do
    {
      :incomplete,
      expect
      |> Enum.reduce(
           0,
           fn c, acc ->
             acc * 5 + @incomplete[c]
           end
         )
    }
  end

  def eat([c | tail], [c | expect]) do
    eat(tail, expect)
  end

  def eat([c | tail], expect) do
    if Map.has_key?(@pairs, c) do
      eat(tail, [@pairs[c] | expect])
    else
      {:corrupt, @corrupt[c]}
    end
  end

  def score_corrupted(chunk) do
    eat(chunk, [])
  end

  def input() do
    File.read!("data/day10.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Stream.map(fn s -> String.split(s, "", trim: true) end)
  end

  def processed(filter_key) do
    input()
    |> Stream.map(&score_corrupted/1)
    |> Stream.filter(fn {type, _val} -> type == filter_key end)
    |> Stream.map(fn {_type, val} -> val end)
    |> Enum.to_list()
  end

  def part1() do
    processed(:corrupt)
    |> Enum.sum()
  end

  def part2() do
    scores = processed(:incomplete)
             |> Enum.sort()
    Enum.at(scores, div(length(scores), 2))
  end
end
