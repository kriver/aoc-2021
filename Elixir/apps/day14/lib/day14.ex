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
    template = String.split(start, "", trim: true)
    element_counts = template
                     |> Enum.frequencies()
    pair_counts = template
                  |> Enum.chunk_every(2, 1, :discard)
                  |> Enum.map(&List.to_tuple/1)
                  |> Enum.frequencies()
    rules = Enum.reduce(
      tail,
      %{},
      fn {from, to}, acc ->
        Map.put(
          acc,
          String.split(from, "", trim: true)
          |> List.to_tuple(),
          to
        )
      end
    )
    {element_counts, pair_counts, rules}
  end

  def evolve(ec, pc, rules) do
    pc
    |> Map.to_list()
    |> Enum.reduce(
         {ec, %{}},
         fn {{a, c}, cnt}, {nec, npc} ->
           b = rules[{a, c}]
           {
             nec
             |> Map.update(b, cnt, &(&1 + cnt)),
             npc
             |> Map.update({a, b}, cnt, &(&1 + cnt))
             |> Map.update({b, c}, cnt, &(&1 + cnt))
           }
         end
       )
  end

  def evolve(ec, _pc, _rules, 0), do: ec
  def evolve(ec, pc, rules, generations) do
    {nec, npc} = evolve(ec, pc, rules)
    evolve(nec, npc, rules, generations - 1)
  end

  def evolve(generations) do
    {ec, pc, rules} = input()
    evolve(ec, pc, rules, generations)
  end

  def score(ec) do
    {minimum, maximum} = ec
                         |> Map.to_list()
                         |> Enum.reduce(
                              fn {_elem, count}, {minimum, maximum} ->
                                {min(count, minimum), max(count, maximum)}
                              end
                            )
    maximum - minimum
  end
end
