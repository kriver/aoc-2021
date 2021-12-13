defmodule Day4 do
  @moduledoc """
  Documentation for `Day4`.
  """

  defmodule Board do
    @dim 5

    @type coord :: Tuple.t(integer, integer)

    @enforce_keys [:id]
    defstruct [
      id: -1,
      numbers: %{},
      rows: List.duplicate(0, @dim),
      columns: List.duplicate(0, @dim),
      step: 0,
      score: 0
    ]
    @type t :: %__MODULE__{
                 id: integer,
                 numbers: Map.t(integer, coord),
                 rows: List.t(integer),
                 columns: List.t(integer),
                 step: integer,
                 score: integer,
               }

    @spec new(List.t(String.t()), integer) :: Board
    def new(lines, id) do
      lines
      |> Enum.with_index
      |> Enum.reduce(
           %Board{id: id},
           fn {row, y}, board ->
             row
             |> String.split()
             |> Enum.with_index
             |> Enum.reduce(
                  board,
                  fn {value, x}, board2 ->
                    Board.add_number(board2, String.to_integer(value), {x, y})
                  end
                )
           end
         )
    end

    @spec add_number(Board, integer, coord) :: Board
    def add_number(board, number, pos) do
      %Board{
        board |
        numbers: Map.put(board.numbers, number, pos)
      }
    end

    @spec update_board(Board, integer, integer) :: Board
    def update_board(board, number, step)  do
      {x, y} = Map.get(board.numbers, number)
      new_board = %Board{
        board |
        numbers: Map.delete(board.numbers, number),
        rows: List.update_at(board.rows, y, &(&1 + 1)),
        columns: List.update_at(board.columns, x, &(&1 + 1)),
      }
      if @dim in new_board.rows or @dim in new_board.columns do
        # board finished, calculate score
        %Board{
          new_board |
          score: number * Enum.sum(Map.keys(new_board.numbers)),
          step: step
        }
      else
        new_board
      end
    end

    @spec play_number(Board, integer, integer) :: Board
    def play_number(board, number, step) do
      case board do
        # scored board
        %Board{score: score} when score > 0 -> board
        # not yet scored board with matching number
        %Board{
          score: 0,
          numbers: %{
            ^number => _
          }
        } -> Board.update_board(board, number, step)
        # catch all for non-matched number
        _ -> board
      end
    end
  end # module Board

  @spec play_number(integer, integer, List.t(Board)) :: List.t(Board)
  def play_number(number, step, boards) do
    boards
    |> Enum.map(fn board -> Board.play_number(board, number, step) end)
  end

  @spec play() :: List.t(Board)
  def play() do
    lines = File.read!("data/day4.txt")
            |> String.split("\n")
    play_order = lines
                 |> Enum.at(0)
                 |> String.split(",")
                 |> Enum.map(&String.to_integer/1)
    boards = lines
             |> Stream.drop(1)
             |> Stream.filter(fn s -> String.length(s) != 0 end)
             |> Stream.chunk_every(5)
             |> Stream.with_index()
             |> Stream.map(fn {lines, id} -> Board.new(lines, id) end)
             |> Enum.to_list()
    play_order
    |> Stream.with_index()
    |> Enum.reduce(boards, fn {number, step}, board -> play_number(number, step, board) end)
    |> Enum.sort_by(fn board -> board.step end)
  end
end
