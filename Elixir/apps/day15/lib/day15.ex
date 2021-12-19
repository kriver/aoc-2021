defmodule Day15 do
  @moduledoc """
  Documentation for `Day15`.
  """

  @delta [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]
  @dim 100
  @multiply 5

  def input() do
    File.read!("data/day15.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Stream.with_index()
    |> Enum.reduce(
         %{},
         fn {line, y}, acc ->
           String.split(line, "", trim: true)
           |> Stream.with_index()
           |> Enum.reduce(
                acc,
                fn {risk, x}, acc ->
                  Map.put(acc, {x, y}, String.to_integer(risk))
                end
              )
         end
       )
  end

  def add_sorted([], new_rp), do: [new_rp]
  def add_sorted([{risk, pos} | queue], {new_risk, new_pos}) when new_risk <= risk do
    [{new_risk, new_pos}, {risk, pos}] ++ queue
  end
  def add_sorted([rp | queue], new_rp), do: [rp | add_sorted(queue, new_rp)]

  def find_path([{total_risk, current} | _q], _v, _mr, _r, dest) when current == dest, do: total_risk
  def find_path([{total_risk, {x, y}} | queue], visited, max_risks, risks, dest) do
    {nq, nv, nmr} = @delta
                    |> Enum.reduce(
                         {queue, visited, max_risks},
                         fn {dx, dy}, {q, v, mr} ->
                           new_pos = {x + dx, y + dy}
                           if Map.has_key?(risks, new_pos) and !MapSet.member?(v, new_pos) do
                             new_risk = total_risk + risks[new_pos]
                             {
                               add_sorted(q, {new_risk, new_pos}),
                               MapSet.put(v, new_pos),
                               Map.update(mr, new_pos, new_risk, &(min(&1, new_risk)))
                             }
                           else
                             {q, v, mr}
                           end
                         end
                       )
    find_path(nq, nv, nmr, risks, dest)
  end
  def find_path(risks, start, dest) do
    visited = MapSet.new([start])
    queue = [{0, start}]
    find_path(queue, visited, %{}, risks, dest)
  end

  def find_path() do
    input()
    |> find_path({0, 0}, {@dim - 1, @dim - 1})
  end

  def  find_path_big() do
    input()
    |> Map.to_list()
    |> Enum.reduce(
         %{},
         fn {{x, y}, r}, acc ->
           Enum.to_list(0..@multiply - 1)
           |> Enum.reduce(
                acc,
                fn my, acc ->
                  Enum.to_list(0..@multiply - 1)
                  |> Enum.reduce(
                       acc,
                       fn mx, acc ->
                         Map.put(acc, {mx * @dim + x, my * @dim + y}, rem(r - 1 + mx + my, 9) + 1)
                       end
                     )
                end
              )
         end
       )
    |> find_path({0, 0}, {@multiply * @dim - 1, @multiply * @dim - 1})
  end
end
