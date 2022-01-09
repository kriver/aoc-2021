defmodule Day7 do
  @moduledoc """
  Documentation for `Day7`.
  """

  defp group_counted(data) do
    data
    |> Enum.reduce(
         %{},
         fn n, acc ->
           Map.update(acc, n, 1, &(&1 + 1))
         end
       )
  end

  defp calculate_fuel(positions, start_f, fuel_f) do
    m = start_f.(positions)
    positions
    |> group_counted()
    |> Map.to_list()
    |> Enum.reduce(0, fn {p, cnt}, acc -> acc + cnt * fuel_f.(p, m) end)
  end

  # Calculates the low median of the data; i.e. not averaging in case of an even
  # number of data elements.
  def median(data) do
    sorted = Enum.sort(data)
    Enum.at(sorted, div(length(sorted), 2))
  end

  def average(data) do
    {sum, cnt} = Enum.reduce(data, {0, 0}, fn p, {sum, cnt} -> {sum + p, cnt + 1}  end)
    div(sum, cnt)
  end

  def constant_fuel(pos, dest) do
    abs(dest - pos)
  end

  def incremental_fuel(pos, dest) do
    delta = abs(dest - pos)
    div(delta * (delta + 1), 2)
  end

  # Shortcut compared to Python implementation; i.e. not checking left and right
  # from starting position (with integer division rounding it might be
  # necessary)
  def run(start_f, fuel_f) do
    File.read!("data/day7.txt")
    |> String.trim()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
    |> calculate_fuel(start_f, fuel_f)
  end
end
