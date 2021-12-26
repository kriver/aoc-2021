defmodule Day19 do
  @moduledoc """
  Documentation for `Day19`.
  """

  require Matrex

  @scanner ~r/^--- scanner (\d+) ---$/

  @rotations [
    Matrex.new([[-1, 0, 0], [0, -1, 0], [0, 0, 1]]),
    Matrex.new([[-1, 0, 0], [0, 1, 0], [0, 0, -1]]),
    Matrex.new([[1, 0, 0], [0, -1, 0], [0, 0, -1]]),
    Matrex.new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
    Matrex.new([[-1, 0, 0], [0, 0, -1], [0, -1, 0]]),
    Matrex.new([[-1, 0, 0], [0, 0, 1], [0, 1, 0]]),
    Matrex.new([[1, 0, 0], [0, 0, -1], [0, 1, 0]]),
    Matrex.new([[1, 0, 0], [0, 0, 1], [0, -1, 0]]),
    Matrex.new([[0, -1, 0], [-1, 0, 0], [0, 0, -1]]),
    Matrex.new([[0, -1, 0], [1, 0, 0], [0, 0, 1]]),
    Matrex.new([[0, 1, 0], [-1, 0, 0], [0, 0, 1]]),
    Matrex.new([[0, 1, 0], [1, 0, 0], [0, 0, -1]]),
    Matrex.new([[0, -1, 0], [0, 0, -1], [1, 0, 0]]),
    Matrex.new([[0, -1, 0], [0, 0, 1], [-1, 0, 0]]),
    Matrex.new([[0, 1, 0], [0, 0, -1], [-1, 0, 0]]),
    Matrex.new([[0, 1, 0], [0, 0, 1], [1, 0, 0]]),
    Matrex.new([[0, 0, -1], [-1, 0, 0], [0, 1, 0]]),
    Matrex.new([[0, 0, -1], [1, 0, 0], [0, -1, 0]]),
    Matrex.new([[0, 0, 1], [-1, 0, 0], [0, -1, 0]]),
    Matrex.new([[0, 0, 1], [1, 0, 0], [0, 1, 0]]),
    Matrex.new([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]),
    Matrex.new([[0, 0, -1], [0, 1, 0], [1, 0, 0]]),
    Matrex.new([[0, 0, 1], [0, -1, 0], [1, 0, 0]]),
    Matrex.new([[0, 0, 1], [0, 1, 0], [-1, 0, 0]])
  ]

  def parse_beacon([]), do: []
  def parse_beacon([line | lines]) do
    [
      line
      |> String.split(",")
      |> Enum.map(&String.to_integer/1)
      | parse_beacon(lines)
    ]
  end

  def parse_scanner([line | lines]) do
    [id | _] = Regex.run(@scanner, line, capture: :all_but_first)
    beacon = parse_beacon(lines)
             |> Matrex.new()
    {
      String.to_integer(id),
      @rotations
      |> Enum.map(fn r -> Matrex.dot(beacon, r) end)
    }
  end

  def parse([]), do: []
  def parse(lines) do
    lines
    |> Enum.chunk_while(
         [],
         fn
           "", acc -> {:cont, Enum.reverse(acc), []}
           line, acc -> {:cont, [line | acc]}
         end,
         fn _acc -> {:cont, []} end
       )
    |> Enum.map(&parse_scanner/1)
  end

  def input() do
    File.read!("data/day19.txt")
    |> String.split("\n")
    |> parse()
  end

  def manhattan_distance(_a, []), do: []
  def manhattan_distance(a, [b | b_tail]) do
    [manhattan_distance(a, b) | manhattan_distance(a, b_tail)]
  end
  def manhattan_distance(a, b) do
    Matrex.subtract(a, b)
    |> Matrex.apply(:abs)
    |> Matrex.sum()
  end

  def overlaps(_a, []), do: false
  def overlaps(a, [b | b_tail]) do
    overlaps(a, b) or overlaps(a, b_tail)
  end
  def overlaps({_id_a, [a | _a_tail]}, {_id_b, b}) do
    # check one of 'a' against all of 'b'
    overlaps(a, b)
  end
  # two matrices with rows of beacon coordinates
  def overlaps(a, b) do
    b_rows = Matrex.list_of_rows(b)
    Matrex.list_of_rows(a)
    |> Enum.reduce(
         [],
         fn row, acc ->
           [manhattan_distance(row, b_rows) | acc]
         end
       )
    |> List.flatten()
    |> Enum.frequencies()
    |> Map.filter(fn {_dist, cnt} -> cnt >= 12 end)
    |> Map.keys()
    |> Enum.fetch(0)
    # FIXME should return b + distance
  end

  def combine(combined, other, distance) do

  end

  def solve(combined, solved, []), do: combined
  def solve(combined, solved, [u | unsolved]) do
    {nc, ns} = solved
               |> Enum.reduce(
                    {combined, []},
                    fn s, {combined, ns} ->
                      case overlaps(s, u) do
                        {:ok, distance} ->
                          # FIXME combine
                          {combine(combined, [], distance), [u | [s | ns]]}
                        # FIXME break out of reduce
                        :error -> {combined, [s | ns]}
                      end
                    end
                  )
    # FIXME what if 'u' not solved?
    solve(nc, ns, unsolved)
  end

  def solve() do
    [first | rest] = input()
    single_rot = first
                 |> Enum.at(0)
                 |> Matrex.to_list_of_lists()
    solve(MapSet.new(single_rot), [first], rest)
  end
end
