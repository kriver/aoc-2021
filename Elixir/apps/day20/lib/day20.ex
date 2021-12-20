defmodule Day20 do
  @moduledoc """
  Documentation for `Day20`.
  """

  @dim 100

  def lit(c), do: (if c == "#", do: 1, else: 0)

  def input() do
    [rules | lines] = File.read!("data/day20.txt")
                      |> String.split("\n")
    {
      rules
      |> String.split("", trim: true)
      |> Stream.map(&lit/1)
      |> Stream.with_index()
      |> Enum.reduce(
           %{},
           fn {l, i}, acc ->
             Map.put(acc, i, l)
           end
         ),
      lines
      |> Stream.filter(&(String.length(&1) > 0))
      |> Stream.with_index()
      |> Enum.reduce(
           %{},
           fn {line, y}, acc ->
             String.split(line, "", trim: true)
             |> Stream.with_index()
             |> Enum.reduce(
                  acc,
                  fn {c, x}, acc ->
                    Map.put(acc, {x, y}, lit(c))
                  end
                )
           end
         )
    }
  end

  def new_value(image, algo, {x, y}, default) do
    algo[
        -1..1
        |> Enum.reduce(
             0,
             fn dy, idx ->
               -1..1
               |> Enum.reduce(
                    idx,
                    fn dx, idx ->
                      idx * 2 + Map.get(image, {x + dx, y + dy}, algo[default])
                    end
                  )
             end
           )
        ]
  end

  def evolve(image, algo, delta, default) do
    d = delta + 1
    -d..(@dim + d - 1)
    |> Enum.reduce(
         %{},
         fn y, acc ->
           -d..(@dim + d - 1)
           |> Enum.reduce(
                acc,
                fn x, acc ->
                  Map.put(acc, {x, y}, new_value(image, algo, {x, y}, default))
                end
              )
         end
       )
  end

  def evolve(image, _algo, _delta, _default, 0), do: image
  def evolve(image, algo, delta, default, generation) do
    image = evolve(image, algo, delta, default)
    evolve(image, algo, delta + 1, (if algo[default] == 1, do: 511, else: 0), generation - 1)
  end

  def evolve(generations) do
    {algo, image} = input()
    delta = 0
    {default, _} = algo
                   |> Map.to_list()
                   |> Stream.filter(fn {_idx, lit} -> lit == 0 end)
                   |> Enum.at(0)
    evolve(image, algo, delta, default, generations)
  end

  def lit_pixels(image) do
    image
    |> Map.values()
    |> Enum.filter(fn v -> v == 1 end)
    |> Enum.count()
  end
end
