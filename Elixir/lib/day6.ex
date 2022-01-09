defmodule Day6 do
  @moduledoc """
  Documentation for `Day6`.
  """

  def evolve(grouped_fish, 0) do
    grouped_fish
    |> Map.values()
    |> Enum.sum()
  end

  def evolve(grouped_fish, days) do
    grouped_fish
    |> Enum.to_list()
    |> Enum.reduce(
         %{},
         fn {days, cnt}, acc ->
           case days do
             0 ->
               acc
               |> Map.update(6, cnt, &(&1 + cnt))
               |> Map.put(8, cnt)
             _ -> Map.update(acc, days - 1, cnt, &(&1 + cnt))
           end
         end
       )
    |> evolve(days - 1)
  end

  def group_counted(data) do
    data
    |> Enum.reduce(
         %{},
         fn n, acc ->
           Map.update(acc, n, 1, &(&1 + 1))
         end
       )
  end

  def count_fish(fish, days) do
    fish
    |> group_counted()
    |> evolve(days)
  end

  def run(days) do
    File.read!("data/day6.txt")
    |> String.trim()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
    |> count_fish(days)
  end
end
