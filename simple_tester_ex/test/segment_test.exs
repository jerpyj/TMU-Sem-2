defmodule SegmentTest do
  use ExUnit.Case
  doctest Segment

  test "test_1_small" do 
    k = 10
    nlines = 5
    :rand.seed(:exsss, 42 + nlines)
    {npassed, all_res} = run_tests(nlines, k)
    File.write("ex_log.txt", "SMALL - 5 LINES:\n" <> all_res)
    assert npassed == k
  end

  test "test_2_medium" do
    k = 5
    nlines = 10
    :rand.seed(:exsss, 42 + nlines)
    {npassed, all_res} = run_tests(nlines, k)
    File.write("ex_log.txt", "MEDIUM - 10 LINES:\n" <> all_res, [:append])
    assert npassed == k
  end

  test "test_3_large" do 
    k = 3
    nlines = 20
    :rand.seed(:exsss, 42 + nlines)
    {npassed, all_res} = run_tests(nlines, k)
    File.write("ex_log.txt", "LARGE - 20 LINES:\n" <> all_res, [:append])
    assert npassed == k
  end

  test "test_4_mega" do
    k = 1
    nlines = 50
    :rand.seed(:exsss, 42 + nlines)
    {npassed, all_res} = run_tests(nlines, k)
    File.write("ex_log.txt", "MEGA - 50 LINES:\n" <> all_res, [:append])
    assert npassed == k
  end

  test "test_5_giga" do 
    k = 1
    nlines = 100
    :rand.seed(:exsss, 42 + nlines)
    {npassed, all_res} = run_tests(nlines, k)
    File.write("ex_log.txt", "GIGA - 100 LINES:\n" <> all_res, [:append])
    assert npassed == k
  end


  def run_tests(nlines, ntests) do

    tests = for n <- 1..ntests, do: gen_inputs(nlines)

    sols  = for {n, {start, finish, lines}} <- Enum.zip(1..ntests, tests) do 
      {time, sol} = :timer.tc(fn -> Segment.segment start, finish, lines end)
      test_result = verify_sol(sol, start, finish, lines)
      res_string = "  Test #{n}: #{inspect start}, #{inspect finish}, #{inspect lines, limit: :infinity}\n"
      res_string = res_string <> "    You returned #{inspect sol}\n"
      res_string = res_string <> "    #{test_result}\n"
      {time, test_result, res_string}
    end

    npassed = Enum.filter(sols, fn {_, res, _} -> res == "PASSED!" end) |> length
    total_time = Enum.map(sols, fn {t, _, _} -> t end) |> Enum.sum
    all_res = Enum.map(sols, fn {_, _, rs} -> rs end) |> Enum.join("")
    all_res = all_res <> "  Passed #{npassed}/#{ntests} tests in #{div(total_time, 1000)} ms\n\n"

    {npassed, all_res}

  end

  def gen_inputs(nlines) do

    start = {Enum.random(-10..10), Enum.random(-10..10)}
    lines = for _ <- 1..nlines, do: Enum.random(1..10)
    dirs = for _ <- 1..nlines, do: ("UDLR" |> String.graphemes |> Enum.random) 
    
    tmp = [start | (Enum.zip(lines, dirs) |> Enum.map(point_map()))]
    finish = Enum.reduce tmp, fn {x, y}, {dx, dy} -> {x+dx, y+dy} end
    {start, finish, lines}

  end

  def verify_sol(sol, _, _, _) when not is_binary(sol) do
    "FAILED: Return type must be String."
  end

  def verify_sol(sol, start, finish, lines) do

    p2 = [start | (Enum.zip(lines, String.graphemes(sol)) |> Enum.map(point_map()))]
      |> Enum.reduce(fn {x, y}, {dx, dy} -> {x+dx, y+dy} end)

    cond do
      String.length(sol) != length lines -> 
        "FAILED: Solution length must match number of lines."
      not Enum.all?(String.graphemes(sol), fn c -> String.contains?("UDLR", c) end) ->
        "FAILED: Solution should only contain U, D, R, L."
      finish != p2 -> "FAILED: Finish point incorrect."      
      true -> "PASSED!"
    end
  end 

  def point_map do
    fn 
      {x, "U"} -> {0, x}
      {x, "D"} -> {0, -x}
      {x, "L"} -> {-x, 0}
      {x, "R"} -> {x, 0}
    end
  end 

end