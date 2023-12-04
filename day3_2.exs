defmodule Day3 do
  alias Protocol.UndefinedError

  def solve(input) do
    all_chars =
      String.split(input, "\n", trim: true)
      |> Enum.map(fn s -> String.graphemes(s) |> Enum.with_index() end)

    all_chars
    |> Enum.with_index()
    |> Enum.map(fn {s, i} -> run_reduce(s, i, all_chars) end)
    |> Enum.sum()
  end

  def run_reduce(input, y, all_chars) do
    Enum.reduce(input, 0, fn {c, x}, acc -> parse_char({x, y}, c, acc, all_chars) end)
  end

  def parse_char(pos, c, acc, all_chars)
      when c in ["*"] do
    check_touching_chars(pos, acc, all_chars)
  end

  def parse_char(_, _, acc, _) do
    acc
  end

  def check_touching_chars({x, y}, acc, all_chars) do
    {touching_acc, amount_for_gears, _} =
      0..2
      |> Enum.flat_map(fn iy -> 0..2 |> Enum.map(fn ix -> {x + ix - 1, y + iy - 1} end) end)
      |> Enum.reduce({1, 0, []}, fn {ix, iy}, {acc, amount_for_gears, visited} ->
        if {ix, iy} in visited do
          {acc, amount_for_gears, visited}
        else
          get_char_value({ix, iy}, {acc, amount_for_gears, visited}, all_chars)
        end
      end)

    if amount_for_gears == 2 do
      acc + touching_acc
    else
      acc
    end
  end

  def get_char_value({x, y}, {acc, amount_for_gears, visited}, all_chars) do
    try do
      {ch, _} = all_chars |> Enum.at(y) |> Enum.at(x)

      case Integer.parse(ch) do
        {v, ""} ->
          get_full_num(v, {x, y}, {acc, amount_for_gears, visited}, Enum.at(all_chars, y))

        _ ->
          {acc, amount_for_gears, visited}
      end
    rescue
      _ in UndefinedError -> {acc, amount_for_gears, visited}
    end
  end

  def get_full_num(v, {x, y}, {acc, amount_for_gears, visited}, current_level_chars) do
    to_left = get_until_period_offset(x, current_level_chars, -1)

    all_nums =
      Enum.reverse(get_until_period_offset(x, current_level_chars, 1)) ++
        [v] ++ to_left

    full_num =
      all_nums
      |> Enum.with_index()
      |> Enum.reduce(0, fn {v, i}, acc -> acc + v * 10 ** i end)

    new_visited =
      (x - Enum.count(to_left))..(x - Enum.count(to_left) + Enum.count(all_nums))
      |> Enum.map(fn x -> {x, y} end)

    {acc * full_num, amount_for_gears + 1, visited ++ new_visited}
  end

  def get_until_period_offset(x, current_level_chars, offset) do
    try do
      {ch, _} = Enum.at(current_level_chars, x + offset)

      case Integer.parse(ch) do
        {v, ""} -> [v | get_until_period_offset(x + offset, current_level_chars, offset)]
        _ -> []
      end
    rescue
      _ in MatchError -> []
    end
  end
end

{:ok, conts} = File.read("./input/day3")
conts |> Day3.solve() |> IO.puts()
