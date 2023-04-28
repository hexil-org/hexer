defmodule HexerBackend.Parser do
  import NimbleParsec

  die_value = integer(min: 1, max: 6)

  roll_value =
    ignore(string("("))
    |> concat(die_value |> unwrap_and_tag(:low))
    |> ignore(string("+"))
    |> concat(die_value |> unwrap_and_tag(:high))
    |> ignore(string(")"))

  roll =
    string("R")
    |> replace(:roll)
    |> unwrap_and_tag(:verb)
    |> concat(roll_value |> tag(:what))

  q_component =
    ascii_string([?a..?z], min: 1)
    # TODO: actually implement instead of constant
    |> replace(3)

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
