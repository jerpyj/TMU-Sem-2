defmodule Segment do
  def segment(start, finish, lines, result \\ "") do
    case lines do
      [] ->
        if start == finish, do: result, else: nil

      [head | tail] ->
        # Calculate total potential reach once per level
        remaining_sum = Enum.sum(tail)
        dist = abs(elem(start, 0) - elem(finish, 0)) + abs(elem(start, 1) - elem(finish, 1))

        # 1. Pruning: If we can't reach OR the parity is wrong, it's a flop
        if dist > head + remaining_sum or rem(dist + head + remaining_sum, 2) != 0 do
          nil
        else
          check_dir(start, finish, head, tail, result)
        end
    end
  end

  defp check_dir(start, finish, head, tail, result) do
    # All the vibes we can take
    directions = [
      {"U", {elem(start, 0), elem(start, 1) + head}},
      {"D", {elem(start, 0), elem(start, 1) - head}},
      {"L", {elem(start, 0) - head, elem(start, 1)}},
      {"R", {elem(start, 0) + head, elem(start, 1)}}
    ]

    # 2. Heuristic Sorting: Try moves that get us closer to the finish FIRST
    sorted_dirs = Enum.sort_by(directions, fn {_, {nx, ny}} ->
      abs(nx - elem(finish, 0)) + abs(ny - elem(finish, 1))
    end)

    Enum.find_value(sorted_dirs, fn {dir_char, next_point} ->
      segment(next_point, finish, tail, result <> dir_char)
    end)
  end
end