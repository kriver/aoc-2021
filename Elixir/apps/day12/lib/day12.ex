defmodule Day12 do
  @moduledoc """
  Documentation for `Day12`.
  """

  def add(map, [a | [b | _]]) do
    map = if b != "start", do: Map.update(map, a, [b], fn v -> [b | v] end), else: map
    if a != "start", do: Map.update(map, b, [a], fn v -> [a | v] end), else: map
  end

  def input() do
    File.read!("data/day12.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Stream.map(fn line -> String.split(line, "-") end)
    |> Enum.reduce(%{}, fn duo, acc -> add(acc, duo) end)
  end

  def is_lowercase(s) do
    String.match?(s, ~r'^[a-z]*$')
  end

  # return true if not allowed
  def only_once?(pos, path, twice) do
    {is_lowercase(pos) and Map.has_key?(path, pos), twice}
  end

  # return true if not allowed
  def only_one_twice?(pos, path, twice) do
    {only_once, _} = only_once?(pos, path, twice)
    {only_once and twice, (if only_once, do: true, else: twice)}
  end

  def do_the_walk(_edges, _twice, [], _path, paths, _is_allowed?) do
    paths
  end

  def do_the_walk(edges, twice, ["end" | tail], path, paths, is_allowed?) do
    do_the_walk(edges, twice, tail, path, [Map.put(path, "end", 1) | paths], is_allowed?)
  end

  require Logger

  def do_the_walk(edges, twice, [pos | tail], path, paths, is_allowed?) do
    {not_allowed, new_twice} = is_allowed?.(pos, path, twice)
    paths = if not_allowed do
      paths
    else
      new_path = Map.update(path, pos, 1, &(&1 + 1))
      do_the_walk(edges, new_twice, edges[pos], new_path, paths, is_allowed?)
    end
    do_the_walk(edges, twice, tail, path, paths, is_allowed?)
  end

  def paths_once() do
    edges = input()
    paths = do_the_walk(edges, false, ["start"], %{}, [], &only_once?/3)
    length(paths)
  end

  def paths_one_twice() do
    edges = input()
    paths = do_the_walk(edges, false, ["start"], %{}, [], &only_one_twice?/3)
    length(paths)
  end
end
