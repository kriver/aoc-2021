defmodule Day21 do
  @moduledoc """
  Documentation for `Day21`.
  """

  @faces [1, 2, 3]
  @moves (for x <- @faces, y <- @faces, z <- @faces, do: [x, y, z])
         |> Enum.map(&Enum.sum/1)
         |> Enum.frequencies()
         |> Map.to_list()

  require Logger

  def play_d100(_player, _p1, _p2, s1, s2, _die, rolls) when s1 >= 1000, do: rolls * s2
  def play_d100(_player, _p1, _p2, s1, s2, _die, rolls) when s2 >= 1000, do: rolls * s1
  def play_d100(2, p1, p2, s1, s2, die, rolls) do
    p2 = rem(p2 + 3 * die + 6, 10)
    s2 = s2 + p2 + 1
    die = rem(die + 3, 100)
    play_d100(1, p1, p2, s1, s2, die, rolls + 3)
  end
  def play_d100(1, p1, p2, s1, s2, die, rolls) do
    p1 = rem(p1 + 3 * die + 6, 10)
    s1 = s1 + p1 + 1
    die = rem(die + 3, 100)
    play_d100(2, p1, p2, s1, s2, die, rolls + 3)
  end
  def play_d100(pos) do
    start_player = 1
    {p1, p2} = pos
    {s1, s2} = {0, 0}
    die = 0
    rolls = 0
    play_d100(start_player, p1 - 1, p2 - 1, s1, s2, die, rolls)
  end

  def play_d3(_player, finished, []) do
    finished
    |> Map.to_list()
    |> Enum.reduce(
         {0, 0},
         fn {{_, _, s1, s2}, c}, {c1, c2} ->
           if s1 > s2, do: {c1 + c, c2}, else: {c1, c2 + c}
         end
       )
    |> Tuple.to_list()
    |> Enum.max()
  end
  def play_d3(player, finished, universes) do
    {nf, nu} = universes
               |> Enum.reduce(
                    {finished, %{}},
                    fn {{p1, p2, s1, s2}, uni_cnt}, {new_finished, new_universes} ->
                      @moves
                      |> Enum.reduce(
                           {new_finished, new_universes},
                           fn {move, move_cnt}, {new_finished, new_universes} ->
                             {np1, np2, ns1, ns2, nc} = if player == 1 do
                               np1 = rem(p1 + move, 10)
                               {np1, p2, s1 + np1 + 1, s2, uni_cnt * move_cnt}
                             else
                               np2 = rem(p2 + move, 10)
                               {p1, np2, s1, s2 + np2 + 1, uni_cnt * move_cnt}
                             end
                             if ns1 >= 21 or ns2 >= 21 do
                               {
                                 Map.update(new_finished, {np1, np2, ns1, ns2}, nc, &(&1 + nc)),
                                 new_universes
                               }
                             else
                               {
                                 new_finished,
                                 Map.update(new_universes, {np1, np2, ns1, ns2}, nc, &(&1 + nc))
                               }
                             end
                           end
                         )
                    end
                  )
    play_d3(3 - player, nf, Map.to_list(nu))
  end

  def play_d3(pos) do
    {p1, p2} = pos
    universes = Map.to_list(%{{p1 - 1, p2 - 1, 0, 0} => 1})
    play_d3(1, %{}, universes)
  end
  def moves(), do: @moves
end
