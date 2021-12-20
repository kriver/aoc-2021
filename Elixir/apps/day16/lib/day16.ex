defmodule Day16 do
  @moduledoc """
  Documentation for `Day16`.
  """

  @hex %{
    "0" => [0, 0, 0, 0],
    "1" => [0, 0, 0, 1],
    "2" => [0, 0, 1, 0],
    "3" => [0, 0, 1, 1],
    "4" => [0, 1, 0, 0],
    "5" => [0, 1, 0, 1],
    "6" => [0, 1, 1, 0],
    "7" => [0, 1, 1, 1],
    "8" => [1, 0, 0, 0],
    "9" => [1, 0, 0, 1],
    "A" => [1, 0, 1, 0],
    "B" => [1, 0, 1, 1],
    "C" => [1, 1, 0, 0],
    "D" => [1, 1, 0, 1],
    "E" => [1, 1, 1, 0],
    "F" => [1, 1, 1, 1],
  }

  def eat(s, 0, value), do: {value, s}
  def eat([head | tail], num, value) do
    eat(tail, num - 1, value * 2 + head)
  end
  def eat(s, num) do
    eat(s, num, 0)
  end

  def parse_literal([0 | s], value), do: eat(s, 4, value)
  def parse_literal([1 | s], value) do
    {value, s} = eat(s, 4, value)
    parse_literal(s, value)
  end

  def parse_by_bits([]), do: []
  def parse_by_bits(s) do
    {pkt, s} = parse(s)
    [pkt | parse_by_bits(s)]
  end

  def parse(s, 0) do
    {bits, s} = eat(s, 15)
    {sub, s} = Enum.split(s, bits)
    {parse_by_bits(sub), s}
  end

  def parse(s, 1) do
    {packets, s} = eat(s, 11)
    1..packets
    |> Enum.reduce(
         {[], s},
         fn _, {acc, s} ->
           {pkt, s} = parse(s)
           {acc ++ [pkt], s}
         end
       )
  end

  def parse([]), do: {nil, []}
  def parse(s) do
    {version, s} = eat(s, 3)
    {type, s} = eat(s, 3)
    {body, s} = case type do
      4 -> parse_literal(s, 0)
      _ -> {length_type, s} = eat(s, 1)
           parse(s, length_type)
    end
    {{version, type, body}, s}
  end

  def to_pkt(s) do
    {pkt, _s} = parse(s)
    pkt
  end

  def input() do
    File.read!("data/day16.txt")
    |> String.split("\n")
    |> Enum.at(0)
    |> String.split("", trim: true)
    |> Enum.flat_map(fn d -> @hex[d] end)
    |> to_pkt()
  end

  def version({v, 4, _sp}), do: v
  def version({v, _t, sp}) do
    v + Enum.sum(Enum.map(sp, &version/1))
  end

  def version() do
    version(input())
  end

  def evaluate({_v, 0, sp}), do: Enum.sum(Enum.map(sp, &evaluate/1))
  def evaluate({_v, 1, sp}), do: Enum.product(Enum.map(sp, &evaluate/1))
  def evaluate({_v, 2, sp}), do: Enum.min(Enum.map(sp, &evaluate/1))
  def evaluate({_v, 3, sp}), do: Enum.max(Enum.map(sp, &evaluate/1))
  def evaluate({_v, 4, value}), do: value
  def evaluate({_v, 5, [a | [b | _]]}) do
    v1 = evaluate(a)
    v2 = evaluate(b)
    if v1 > v2, do: 1, else: 0
  end
  def evaluate({_v, 6, [a | [b | _]]}) do
    v1 = evaluate(a)
    v2 = evaluate(b)
    if v1 < v2, do: 1, else: 0
  end
  def evaluate({_v, 7, [a | [b | _]]}) do
    v1 = evaluate(a)
    v2 = evaluate(b)
    if v1 == v2, do: 1, else: 0
  end

  def evaluate() do
    evaluate(input())
  end
end
