defmodule HexerBackend.Parser do
  import NimbleParsec

  defp roll_value_from_roll_pair([l, r]) do
    %{low: min(l, r), high: max(l, r)}
  end

  defp letter_to_int(letter) do
    alphabet = ~w(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z)
    Enum.find_index(alphabet, fn c -> c == letter end) + 1
  end

  defp letters_to_int(letters) when is_bitstring(letters) do
    letters
    |> String.upcase()
    |> String.graphemes()
    |> Enum.map(&letter_to_int(&1))
    |> List.foldr(0, fn digit, acc -> digit + 26 * acc end)
  end

  die_value = integer(min: 1, max: 6)

  roll_value =
    ignore(string("("))
    |> concat(die_value)
    |> ignore(string("+"))
    |> concat(die_value)
    |> ignore(string(")"))
    |> wrap()
    |> map({:roll_value_from_roll_pair, []})

  roll =
    string("R")
    |> replace(:roll)
    |> unwrap_and_tag(:verb)
    |> concat(roll_value |> unwrap_and_tag(:what))

  q_component =
    ascii_string([?a..?z], min: 1)
    |> map({:letters_to_int, []})

  r_component = integer(min: 1)

  tile_coordinate =
    unwrap_and_tag(q_component, :q)
    |> concat(r_component |> unwrap_and_tag(:r))

  move_robber =
    string("M")
    |> replace(:move_robber)
    |> unwrap_and_tag(:verb)
    |> concat(tile_coordinate |> tag(:to))

  action = choice([roll, move_robber])

  defparsecp(:parsec_action, action |> eos())

  def parse_action(str) do
    {:ok, result, _, _, _, _} = parsec_action(str)
    {:ok, to_map_deep(result)}
  end

  defp to_map_deep([{k, v} | t]) when is_atom(k) do
    Map.put_new(to_map_deep(t), k, to_map_deep(v))
  end

  defp to_map_deep([]) do
    %{}
  end

  defp to_map_deep(l) when is_list(l) do
    List.to_tuple(l)
  end

  defp to_map_deep(x) do
    x
  end
end
