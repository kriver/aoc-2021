defmodule Day5 do
  @moduledoc """
  Documentation for `Day5`.
  """

  defmodule Coord do
    defstruct [:x, :y]
    @type t :: %__MODULE__{
                 x: integer,
                 y: integer,
               }

    @spec new(String.t()) :: Coord
    def new(line) do
      parsed = String.split(line, ",")
               |> Enum.map(&String.to_integer/1)
      %Coord {
        x: Enum.at(parsed, 0),
        y: Enum.at(parsed, 1)
      }
    end
  end

  defmodule Line do
    defstruct [:start, :end]
    @type t :: %__MODULE__{
                 start: Coord.t(),
                 end: Coord.t(),
               }

    @spec new(String.t()) :: Line
    def new(line) do
      coords = line
               |> String.split(" -> ")
               |> Enum.map(&Coord.new/1)
      %Line {
        start: Enum.at(coords, 0),
        end: Enum.at(coords, 1)
      }
    end

    def is_horizontal(line) do
      line.start.y == line.end.y
    end

    def is_vertical(line) do
      line.start.x == line.end.x
    end

    def is_horz_or_vert(line) do
      is_horizontal(line) or is_vertical(line)
    end

    @spec points(Line.t()) :: List.t(Coord.t())
    def points(line) do
      dx = line.end.x - line.start.x
      dy = line.end.y - line.start.y
      steps = max(abs(dx), abs(dy))
      0..steps
      |> Enum.map(
           fn i -> %Coord{
                     x: line.start.x + div(i * dx, steps),
                     y: line.start.y + div(i * dy, steps)
                   }
           end
         )
    end
  end

  @type field :: Map.t(Coord.t(), integer)
  @spec build_map(Line.t(), field) :: field
  def build_map(line, points) do
    Line.points(line)
    |> Enum.reduce(
         points,
         fn p, acc ->
           Map.update(acc, p, 1, &(&1 + 1))
         end
       )
  end

  def run(allow_diagonal) do
    File.read!("data/day5.txt")
    |> String.split("\n")
    |> Stream.filter(fn s -> String.length(s) != 0 end)
    |> Stream.map(&Line.new/1)
    |> Stream.filter(fn l -> allow_diagonal or Line.is_horz_or_vert(l) end)
    |> Enum.reduce(%{}, &build_map/2)
    |> Map.values()
    |> Stream.filter(&(&1 > 1))
    |> Enum.count()
  end
end
