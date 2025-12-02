defmodule Day2 do
  @input File.read!("input.txt")

  def part1 do
    @input
    |> String.split(",")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&parse_range/1)
    |> Enum.flat_map(&Range.to_list/1)
    |> Enum.reject(&valid?/1)
    |> Enum.sum()
  end

  def part2 do
    @input
    |> String.split(",")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&parse_range/1)
    |> Enum.flat_map(&Range.to_list/1)
    |> Enum.reject(&super_valid?/1)
    |> Enum.sum()
  end

  defp parse_range(range_str) do
    [from, to] =
      range_str
      |> String.split("-")
      |> Enum.map(&String.to_integer/1)

    Range.new(from, to)
  end

  defp valid?(product_id) do
    product_id = Integer.to_string(product_id)
    digits = String.length(product_id)
    {front, back} = String.split_at(product_id, div(digits, 2))
    front != back
  end

  defp super_valid?(product_id) do
    product_id = Integer.to_string(product_id)
    digits = String.length(product_id)

    if digits == 1 do
      true
    else
      invalid =
        Range.new(1, div(digits, 2))
        |> Enum.filter(fn n -> rem(digits, n) == 0 end)
        |> Enum.map(&substring(product_id, &1))
        |> Enum.any?(fn substring ->
          count_occurrences(substring, product_id) == div(digits, String.length(substring))
        end)

      not invalid
    end
  end

  defp count_occurrences(substring, string) do
    string
    |> String.split(substring)
    |> length()
    |> then(fn len -> len - 1 end)
  end

  defp substring(string, n) do
    String.slice(string, 0, n)
  end
end

IO.puts("Part 1: #{Day2.part1()}")
IO.puts("Part 2: #{Day2.part2()}")
