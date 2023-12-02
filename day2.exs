defmodule Day2 do
  def solve(input) do
    String.split(input, "\n", trim: true) |> Enum.reduce(0, &solve_line/2)
  end

  def solve_line(line, acc) do
    [game_id, games] = String.split(line, ": ")

    if are_all_games_good(games) do
      acc + String.to_integer(String.trim_leading(game_id, "Game "))
    else
      acc
    end
  end

  def is_possible(str) do
    case String.split(str, " ") do
      [x, "red"] -> String.to_integer(x) <= 12
      [x, "green"] -> String.to_integer(x) <= 13
      [x, "blue"] -> String.to_integer(x) <= 14
      _ -> false
    end
  end

  def are_all_games_good(line) do
    String.split(line, "; ")
    |> Enum.flat_map(fn set ->
      Enum.map(String.split(set, ", "), &is_possible/1)
    end)
    |> Enum.all?()
  end
end

{:ok, conts} = File.read("./input/day2")
conts |> Day2.solve() |> IO.puts()

# IO.puts(Day2.solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
# Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
# Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
# Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
# Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"))
