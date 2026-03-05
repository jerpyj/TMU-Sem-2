defmodule Segment do
  @moduledoc """
    Add your solver function below.
  """

  def segment(start, finish, lines, result \\ "") do
    case lines do
      [] -> 
        if start == finish, do: result, else: nil

      [head | tail] ->
        checkDir(start, finish, head, tail, result)
    end
  end

  def checkDir(start, finish, head, tail, result) do
    directions = [
      {"U", {elem(start, 0), elem(start, 1) + head}},
      {"D", {elem(start, 0), elem(start, 1) - head}},
      {"L", {elem(start, 0) - head, elem(start, 1)}},
      {"R", {elem(start, 0) + head, elem(start, 1)}}
    ]

    Enum.find_value(directions, fn {dir_char, next_point} ->
      case segment(next_point, finish, tail, result <> dir_char) do
        nil -> nil
        found_path -> found_path
      end
    end)
  end
end


# !Segments class methodsFor: 'as yet unclassified' stamp: 'JeremyPorritt 2/19/2026 21:58'!
# segmentRecursion: start to: end using: lines

#     | remainingDistance totalDistance |

# 	"Check if lines is an empty collection, if it is, check if start is at the end yet"
# 	lines isEmpty ifTrue: [ ^ (start = end) ifTrue: [ '' ] ifFalse: [ nil ] ].
	
# 	"Check if total distance is even possible with provided length"
# 	remainingDistance := (start x - end x) abs + (start y - end y) abs.
# 	totalDistance := lines sum.
# 	remainingDistance > totalDistance ifTrue: [ ^nil ]. "It is not possible to reach whatsoever"
	
# 	"Try each direction if it is possible"
# 	#('U' 'D' 'L' 'R') do: [  :direction |
# 		| movedPoint result |
		
# 		(direction = 'U') ifTrue: [ movedPoint := start x @ (start y + lines first) ].
# 		(direction = 'D') ifTrue: [ movedPoint := start x @ (start y - lines first) ].
# 		(direction = 'L') ifTrue: [ movedPoint := (start x - lines first) @ start y ].
# 		(direction = 'R') ifTrue: [ movedPoint := (start x + lines first) @ start y ].
		
# 		"Recursively call back to the funtion giving the moved point
# 		 as start and the line array exceptfor the first value"
# 		result := self segmentRecursion: movedPoint to: end using: lines allButFirst.
		
# 		"Check if if a path was found, if so, connect the direction string to the result string "
# 		result ifNotNil: [  ^ direction , result ].
		
# 	 ].
# 	^nil! !
