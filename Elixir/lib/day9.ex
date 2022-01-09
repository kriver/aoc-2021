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

  def prepare() do
    height_map = parse_input()
                 |> add_neighbour_heights()
                 |> Map.map(fn {_k, {h, o}} -> {h, length(o)} end)
    low_points = height_map
                 |> Map.filter(fn {_key, {_height, nb}} -> nb == 4 end)
    {height_map, low_points}
  end

  def low_point_risks() do
    {_, low_points} = prepare()
    low_points
    |> Map.values()
    |> Stream.map(fn {height, _} -> height + 1 end)
    |> Enum.sum()
  end

  def valid_neighbours_not_visited(map, {x, y}, visited) do
    @neighbours
    |> Stream.map(fn {dx, dy} -> {x + dx, y + dy} end)
    |> Stream.filter(fn pos -> Map.has_key?(map, pos) end)
    |> Stream.filter(fn pos -> !MapSet.member?(visited, pos) end)
    |> Stream.map(fn pos -> {h, _} = Map.get(map, pos, {9, []});{pos, h} end)
    |> Stream.filter(fn {_pos, h} -> h != 9 end)
    |> Stream.map(fn {pos, _h} -> pos end)
    |> Enum.to_list()
  end

  def visit_one(map, pos, visited) do
    new_visited = MapSet.put(visited, pos)
    grow(
      map,
      valid_neighbours_not_visited(map, pos, new_visited),
      new_visited
    )
  end

  def grow(_map, [], visited) do
    visited
  end

  def grow(map, queue, visited) do
    queue
    |> Enum.reduce(
         visited,
         fn pos, acc -> visit_one(map, pos, acc) end
       )
  end

  def build_basins() do
    {height_map, low_points} = prepare()
    low_points
    |> Map.keys()
    |> Stream.map(fn pos -> grow(height_map, [pos], MapSet.new()) end)
  end

  def basin_sizes() do
    build_basins()
    |> Stream.map(&Enum.count/1)
    |> Enum.to_list()
  end

  def product_largest_basin_sizes(num) do
    basin_sizes()
    |> Enum.sort(&(&1 > &2))
    |> Enum.slice(0..(num - 1))
    |> Enum.reduce(1, fn b, acc -> acc * b end)
  end
end
