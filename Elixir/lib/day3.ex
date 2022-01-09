defmodule Day3 do
  @moduledoc """
  Advent Of Code 2021 - Day 3.
  """

  def power_consumption() do
    File.read!("data/day3.txt")
    |> String.split() # list of lines
    |> Enum.map(fn line -> String.split(line, "", trim: true) end) # list of list of bits
    |> Enum.zip_with(& &1) # transpose
    |> Enum.map(&Enum.frequencies/1) # map of bit counts
    |> Enum.map(fn m -> if m["0"] > m["1"], do: [0, 1], else: [1, 0] end) # split max/min
    |> Enum.zip_with(& &1) # transpose
    |> Enum.map(&Enum.join/1)
    |> Enum.map(fn s -> String.to_integer(s, 2) end)
    |> Enum.reduce(1, &(&1 * &2)) # product of gamma and epsilon rate
  end

  def rating([single], _, _) do
    single # stop when only one element left
  end

  def rating(diagnostic, idx, pred) do
    bit_freq = diagnostic
               |> Enum.zip_with(& &1) # transpose
               |> Enum.map(&Enum.frequencies/1) # map of bit counts
               |> Enum.at(idx)
    bit = if pred.(bit_freq["0"], bit_freq["1"]), do: "0", else: "1"
    diagnostic
    |> Enum.filter(fn bits -> Enum.at(bits, idx) == bit end)
    |> rating(idx + 1, pred)
  end

  def life_support() do
    bit_strings = File.read!("data/day3.txt")
                  |> String.split() # list of lines
                  |> Enum.map(fn line -> String.split(line, "", trim: true) end) # list of list of bits
    oxygen_rating = rating(bit_strings, 0, fn a, b -> a > b end)
                    |> Enum.join()
                    |> String.to_integer(2)
    co2_scrubbing_rating = rating(bit_strings, 0, fn a, b -> a <= b end)
                           |> Enum.join()
                           |> String.to_integer(2)
    oxygen_rating * co2_scrubbing_rating
  end
end
