defmodule Day2 do
  @moduledoc """
  Advent Of Code 2021 - Day 2.
  """

  def parse_line(line) do
    [dir, dist] = String.split(line)
    [dir, String.to_integer(dist)]
  end

  def move_no_aim(step, pos) do
    [horz, vert] = pos
    case step do
      ["forward", x] -> [horz + x, vert]
      ["down", x] -> [horz, vert + x]
      ["up", x] -> [horz, vert - x]
    end
  end

  def move_with_aim(step, pos) do
    [horz, vert, aim] = pos
    case step do
      ["forward", x] -> [horz + x, vert + x * aim, aim]
      ["down", x] -> [horz, vert, aim + x]
      ["up", x] -> [horz, vert, aim - x]
    end
  end

  def run(acc, move) do
    File.read!("data/day2.txt")
    |> String.split("\n")
    |> Stream.filter(fn s -> String.length(s) > 0 end) # filter out empty lines
    |> Stream.map(&parse_line/1) # stream of pairs
    |> Enum.to_list()
    |> Enum.reduce(acc, move) # calculate movement
    |> Enum.slice(0..1) # only take (horz, vert) into account
    |> Enum.reduce(1, &(&1 * &2)) # product of position
  end

  def run_no_aim() do
    run([0, 0], &move_no_aim/2)
  end

  def run_with_aim() do
    run([0, 0, 0], &move_with_aim/2)
  end
end
