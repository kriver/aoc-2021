defmodule Day9 do
  @moduledoc """
  Documentation for `Day9`.
  """

  @neighbours [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]

  # map = {x,y} -> {height, [higher heights from neighbours]}

  def higher_neighbours(map, {x, y}, height) do
    @neighbours
    |> Stream.map(fn {dx, dy} -> {x + dx, y + dy} end)
    |> Stream.map(fn pos -> Map.get(map, pos, {100, []}) end)
    |> Stream.map(fn {h, _o} -> h end)
    |> Enum.filter(fn h -> h > height end)
  end

  def parse_input() do
    File.read!("data/day9.txt")
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
                fn {height, x}, acc -> Map.put(acc, {x, y}, {height, []}) end
              )
         end
       )
  end

  def add_neighbour_heights(map) do
    Map.map(
      map,
      fn {pos, {height, []}} ->
        {height, higher_neighbours(map, pos, height)}
      end
    )
  end

  def low_point_risks() do
    height_map = parse_input()
                 |> add_neighbour_heights()
                 |> Map.map(fn {_k, {h, o}} -> {h, length(o)} end)
    low_points = height_map
                 |> Map.filter(fn {_key, {_height, nb}} -> nb == 4 end)
    low_points
    |> Map.values()
    |> Stream.map(fn {height, _} -> height + 1 end)
    |> Enum.sum()
  end
end
