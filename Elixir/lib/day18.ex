defmodule Day18 do
  @moduledoc """
  Documentation for `Day18`.
  """

  require Logger

  def add(num1, nil), do: num1
  def add(nil, num2), do: num2
  def add(num1, num2), do: {num1, num2}

  def split_rec(num) do
    Logger.debug("Split-L #{inspect(num)}")
    {left, right} = num
    cond do
      is_number(left) and left > 9 ->
        half = div(left, 2)
        {{{half, left - half}, right}, true}
      is_tuple(left) ->
        {left_num, did_split} = split_rec(left)
        if did_split do
          {{left_num, right}, true}
        else
          split_rec_right(num)
        end
      true -> split_rec_right(num)
    end
  end

  def split_rec_right(num) do
    Logger.debug("Split-R #{inspect(num)}")
    {left, right} = num
    cond do
      is_number(right) and right > 9 ->
        half = div(right, 2)
        {{left, {half, right - half}}, true}
      is_tuple(right) ->
        {right_num, did_split} = split_rec(right)
        if did_split do
          {{left, right_num}, true}
        else
          {num, false}
        end
      true -> {num, false}
    end
  end

  def split(num) do
    {num, _did_split} = split_rec(num)
    num
  end

  def add_to_left(num, delta) when is_number(num) do
    num + delta
  end

  def add_to_left({left, right}, delta) do
    {add_to_left(left, delta), right}
  end

  def add_to_right(num, delta) when is_number(num) do
    num + delta
  end

  def add_to_right({left, right}, delta) do
    {left, add_to_right(right, delta)}
  end

  def explode_rec({left, right}, 4) do
    {left, right, 0, true}
  end

  def explode_rec(num, depth) do
    Logger.debug("Explode-L #{depth} #{inspect(num)}")
    {left, right} = num
    if is_tuple(left) do
      {left_delta, right_delta, acc, did_explode} = explode_rec(left, depth + 1)
      if did_explode do
        if right_delta do
          {left_delta, nil, {acc, add_to_left(right, right_delta)}, true}
        else
          {left_delta, right_delta, {acc, right}, true}
        end
      else
        explode_rec_right(num, depth)
      end
    else
      explode_rec_right(num, depth)
    end
  end

  def explode_rec_right(num, depth) do
    Logger.debug("Explode-R #{depth} #{inspect(num)}")
    {left, right} = num
    if is_tuple(right) do
      {left_delta, right_delta, acc, did_explode} = explode_rec(right, depth + 1)
      if did_explode do
        if left_delta do
          {nil, right_delta, {add_to_right(left, left_delta), acc}, true}
        else
          {left_delta, right_delta, {left, acc}, true}
        end
      else
        {nil, nil, num, false}
      end
    else
      {nil, nil, num, false}
    end
  end

  def explode(num) do
    {_, _, num, _} = explode_rec(num, 0)
    num
  end

  def reduce(num) do
    Logger.debug("Reduce #{inspect(num)}")
    {_, _, num, did_explode} = explode_rec(num, 0)
    Logger.debug("Exploded #{did_explode}")
    if did_explode do
      reduce(num)
    else
      {num, did_split} = split_rec(num)
      Logger.debug("Split #{did_split}")
      if did_split do
        reduce(num)
      else
        num
      end
    end
  end

  def add_reduce(num1, num2) do
    reduce(add(num1, num2))
  end

  def magnitude(num) when is_number(num), do: num
  def magnitude({left, right}), do: 3 * magnitude(left) + 2 * magnitude(right)

  def to_tuple(n) when is_number(n), do: n
  def to_tuple(list) when is_list(list) do
    list
    |> Enum.map(&to_tuple/1)
    |> List.to_tuple()
  end
  def to_tuple(line) when is_binary(line) do
    {result, _binding} = Code.eval_string(line)
    to_tuple(result)
  end

  def homework() do
    File.read!("data/day18.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Stream.map(&to_tuple/1)
    |> Enum.reduce(
         nil,
         fn num, acc ->
           Logger.debug("Add #{inspect(acc)} and #{inspect(num)}")
           add_reduce(acc, num)
         end
       )
    |> magnitude()
  end

  def largest_sum(num1, numbers) do
    Enum.reduce(
      numbers,
      0,
      fn num2, acc ->
        Logger.debug("#{acc} --- Sum #{inspect(num1)} and #{inspect(num2)}")
        max(
          magnitude(add_reduce(num1, num2)),
          max(
            magnitude(add_reduce(num2, num1)),
            acc
          )
        )
      end
    )
  end

  def largest_sum([]), do: 0
  def largest_sum([head | tail]) do
    max(largest_sum(head, tail), largest_sum(tail))
  end

  def backside() do
    File.read!("data/day18.txt")
    |> String.split("\n")
    |> Stream.filter(&(String.length(&1) > 0))
    |> Enum.map(&to_tuple/1)
    |> largest_sum()
  end
end
