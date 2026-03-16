defmodule Segment do

  # body recursive

  def segment(start, finish, lines, result \\ "") do
    case lines do
      [] -> 
        if start == finish, do: result, else: nil
      [head | tail] ->
        # Check if its even possible with provided length
        totalDistance = Enum.sum(tail)
        distance = abs(elem(start, 0) - elem(finish, 0)) + abs(elem(start, 1) - elem(finish, 1))

        #if not possible stop the path
        if distance > head + totalDistance do
          nil
        else
          calcDir(start, finish, head, tail, result)
        end
    end
  end

  # Calculate next movements in all the directions
  def calcDir(start, finish, head, tail, result) do
    directions = [
      {"U", {elem(start, 0), elem(start, 1) + head}},
      {"D", {elem(start, 0), elem(start, 1) - head}},
      {"L", {elem(start, 0) - head, elem(start, 1)}},
      {"R", {elem(start, 0) + head, elem(start, 1)}}
    ]

    # Test the directions
    Enum.find_value(directions, fn {direction, movedPoint} ->
      # Recurse into the new direction
      segment(movedPoint, finish, tail, result <> direction)
    end)
  end
end
