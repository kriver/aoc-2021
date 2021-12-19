defmodule Day14 do
  @moduledoc """
  Documentation for `Day14`.
  """

  def input() do
    [{start} | tail] = File.read!("data/day14.txt")
                       |> String.split("\n")
                       |> Stream.filter(&(String.length(&1) > 0))
                       |> Stream.map(fn s -> String.split(s, " -> ") end)
                       |> Stream.map(&List.to_tuple/1)
                       |> Enum.to_list()
    {String.split(start, "", trim: true), Enum.into(tail, %{})}
  end

  def evolve([a], _rules), do: [a]
  def evolve([a | [b | template]], rules) do
    [a, rules[a <> b]] ++ evolve([b | template], rules)
  end

  def evolve(template, _rules, 0), do: template
  def evolve(template, rules, generations) do
    evolve(evolve(template, rules), rules, generations - 1)
  end

  def evolve(generations) do
    {template, rules} = input()
    evolve(template, rules, generations)
  end

  def score(template) do
    {minimum, maximum} = template
                         |> Enum.frequencies()
                         |> Map.to_list()
                         |> Enum.reduce(
                              fn {_elem, count}, {minimum, maximum} ->
                                {min(count, minimum), max(count, maximum)}
                              end
                            )
    maximum - minimum
  end
end
