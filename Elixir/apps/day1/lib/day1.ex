defmodule Day1 do
  @moduledoc """
  Advent Of Code 2021 - Day 1.
  """

  def count_increases(depths, window) do
    depths
    |> Enum.chunk_every(window, 1, :discard) # list of all windows
    |> Enum.map(&Enum.sum/1) # list of all window sums
    |> Enum.chunk_every(2, 1, :discard) # list of pairs of sums
    |> Enum.filter(fn [a, b] -> a < b end) # list of increasing pairs
    |> Enum.count()
  end

  def run(window) do
    File.read!("data/day1.txt")
    |> String.split()
    |> Enum.map(&String.to_integer/1)
    |> count_increases(window)
  end
end
