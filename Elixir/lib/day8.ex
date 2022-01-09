defmodule Day8 do
  @moduledoc """
  Documentation for `Day8`.
  """

  def parse_input() do
    File.read!("data/day8.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) != 0))
    |> Stream.map(
         fn s ->
           String.split(s, ~S' | ')
           |> Enum.map(
                fn io ->
                  String.split(io)
                  # sort characters in pattern strings
                  |> Enum.map(
                       fn pattern ->
                         String.split(pattern, "", trim: true)
                         |> Enum.sort()
                         |> Enum.join()
                       end
                     )
                end
              )
         end
       )
    |> Enum.to_list()
  end

  def count_1478() do
    parse_input()
    |> Stream.flat_map(fn io -> Enum.at(io, 1) end)
    |> Stream.filter(fn o -> MapSet.member?(MapSet.new([2, 3, 4, 7]), String.length(o)) end)
    |> Enum.count()
  end
end
