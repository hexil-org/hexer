defmodule HexerBackend.Parser do
  import NimbleParsec

  die_value = integer(min: 1, max: 6)

  roll_value =
    ignore(string("("))
    |> concat(die_value)
    |> ignore(string("+"))
    |> concat(die_value)
    |> ignore(string(")"))
    |> tag(:what)

  roll =
    string("R")
    |> replace(:roll)
    |> unwrap_and_tag(:verb)
    |> concat(roll_value)

  action = roll

  defparsecp(:parsec_action, action |> eos())

  def parse_action(str) do
    {:ok, result, _, _, _, _} = parsec_action(str)
    {:ok, to_map_deep(result)}
  end

  defp to_map_deep([{k, v} | t]) do
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
