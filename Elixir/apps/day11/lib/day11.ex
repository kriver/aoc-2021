defmodule Day11 do
  @moduledoc """
  Documentation for `Day11`.
  """

  @neighbours [{-1, -1}, {0, -1}, {1, -1}, {-1, 0}, {1, 0}, {-1, 1}, {0, 1}, {1, 1}]

  def input() do
    File.read!("data/day11.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Stream.with_index()
    |> Enum.reduce(
         %{},
         fn {s, y}, acc ->
           String.split(s, "", trim: true)
           |> Stream.map(&String.to_integer/1)
           |> Stream.with_index()
           |> Enum.reduce(
                acc,
                fn {energy, x}, acc -> Map.put(acc, {x, y}, energy) end
              )
         end
       )
  end

  def flash_1(levels, {x, y}) do
    to_increase = @neighbours
                  |> Stream.map(fn {dx, dy} -> {x + dx, y + dy} end)
                  |> Stream.filter(fn pos -> Map.has_key?(levels, pos) end) # only on the map
                  |> Stream.filter(fn pos -> levels[pos] <= 9 end) # only flash once
    levels = to_increase
             |> Enum.reduce(levels, fn pos, acc -> Map.update!(acc, pos, &(&1 + 1)) end)
    to_flash = to_increase
               |> Enum.filter(fn pos -> levels[pos] > 9 end)
    {levels, to_flash}
  end

  def flash(levels, flashed, []) do
    {levels, flashed}
  end

  def flash(levels, flashed, [pos | to_flash]) do
    {levels, new_to_flash} = flash_1(levels, pos)
    flash(levels, [pos | flashed], to_flash ++ new_to_flash)
  end

  def cycle_once(levels, flashed) do
    levels = Map.map(levels, fn {_k, v} -> v + 1 end)
    to_flash = levels
               |> Map.filter(fn {_k, v} -> v > 9 end)
               |> Map.to_list()
               |> Enum.map(fn {k, _v} -> k end)
    {levels, flashed} = flash(levels, flashed, to_flash)
    levels = Map.map(levels, fn {_k, v} -> if v > 9, do: 0, else: v end)
    {levels, flashed}
  end

  def cycle_count(_levels, flashed, 0) do
    flashed
  end

  def cycle_count(levels, flashed, cycles) do
    {levels, flashed} = cycle_once(levels, flashed)
    cycle_count(levels, flashed, cycles - 1)
  end

  def flash_count(cycles) do
    levels = input()
    cycle_count(levels, [], cycles)
    |> Enum.count()
  end

  def cycle_all(levels, cycles) do
    {levels, flashed} = cycle_once(levels, [])
    if length(flashed) == 100 do
      cycles + 1
    else
      cycle_all(levels, cycles + 1)
    end
  end

  def all_flash() do
    levels = input()
    cycle_all(levels, 0)
  end
end
