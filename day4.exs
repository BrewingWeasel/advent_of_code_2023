defmodule Day4 do
  def solve(input) do
    String.split(input, "\n", trim: true) |> Enum.map(&solve_line/1) |> Enum.sum()
  end

  def solve_line(line) do
    [_card_id, cards] = String.split(line, ": ")

    [winning, have] =
      String.split(cards, " | ") |> Enum.map(fn side -> String.split(side, " ", trim: true) end)

    amount_of_winning_nums = have |> Enum.filter(fn num -> num in winning end) |> Enum.count()

    if amount_of_winning_nums > 0 do
      1 * 2 ** (amount_of_winning_nums - 1)
    else
      0
    end
  end
end

{:ok, conts} = File.read("./input/day4")
conts |> Day4.solve() |> IO.puts()
