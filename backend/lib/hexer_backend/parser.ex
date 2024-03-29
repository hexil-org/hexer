defmodule HexerBackend.Parser do
  import NimbleParsec

  defp roll_pair_to_roll_value([l, r]) do
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

  unknown_resource = string("?") |> replace(:unknown_resource)

  unknown_development = string("D?") |> replace(:unknown_development)

  die_value = integer(min: 1)

  roll_value =
    ignore(string("("))
    |> concat(die_value)
    |> ignore(string("+"))
    |> concat(die_value)
    |> ignore(string(")"))
    |> wrap()
    |> map({:roll_pair_to_roll_value, []})

  q_component =
    ascii_string([?a..?z], min: 1)
    |> map({:letters_to_int, []})

  r_component = integer(min: 1)

  corner =
    choice([
      string("n") |> replace(:north),
      string("s") |> replace(:south)
    ])

  border =
    choice([
      string("ne") |> replace(:north_east),
      string("nw") |> replace(:north_west),
      string("w") |> replace(:west)
    ])

  tile_coordinate =
    unwrap_and_tag(q_component, :q)
    |> concat(unwrap_and_tag(r_component, :r))

  vertex_coordinate =
    unwrap_and_tag(q_component, :q)
    |> concat(unwrap_and_tag(r_component, :r))
    |> concat(unwrap_and_tag(corner, :corner))

  edge_coordinate =
    unwrap_and_tag(q_component, :q)
    |> concat(unwrap_and_tag(r_component, :r))
    |> concat(unwrap_and_tag(border, :border))

  player =
    integer(min: 1)
    |> unwrap_and_tag(:player_number)

  resource =
    choice([
      string("B") |> replace(:brick),
      string("G") |> replace(:grain),
      string("L") |> replace(:lumber),
      string("O") |> replace(:ore),
      string("W") |> replace(:wool)
    ])

  resource_formula =
    times(resource |> concat(choice([integer(min: 1), empty() |> replace(1)])) |> wrap, min: 1)
    |> map({List, :to_tuple, []})

  bracketed_resource_fomula =
    ignore(string("("))
    |> concat(resource_formula)
    |> ignore(string(")"))

  village = string("V") |> replace(:village)

  city = string("C") |> replace(:city)

  road = string("R") |> replace(:road)

  structure = choice([village, city, road])

  knight = string("Dk") |> replace(:knight)

  monopoly = string("Dm") |> replace(:monopoly)
  road_building = string("Dr") |> replace(:road_building)
  victory_point = string("Dv") |> replace(:victory_point)
  year_of_plenty = string("Dy") |> replace(:year_of_plenty)

  development = choice([knight, monopoly, road_building, victory_point, year_of_plenty])

  buyable =
    choice([
      replace(empty(), :structure)
      |> unwrap_and_tag(:type)
      |> concat(structure |> unwrap_and_tag(:which)),
      replace(empty(), :development)
      |> unwrap_and_tag(:type)
      |> concat(choice([development, unknown_development]) |> unwrap_and_tag(:which))
    ])

  roll =
    string("R")
    |> replace(:roll)
    |> unwrap_and_tag(:verb)
    |> concat(roll_value |> unwrap_and_tag(:what))

  move_robber =
    string("M")
    |> replace(:move_robber)
    |> unwrap_and_tag(:verb)
    |> concat(tile_coordinate |> tag(:to))

  abandon =
    player
    |> unwrap_and_tag(:who)
    |> concat(string("A") |> replace(:abandon) |> unwrap_and_tag(:verb))
    |> concat(bracketed_resource_fomula |> tag(:what))

  steal =
    string("S")
    |> replace(:steal)
    |> unwrap_and_tag(:verb)
    |> concat(choice([unknown_resource, resource]) |> unwrap_and_tag(:what))
    |> concat(player |> unwrap_and_tag(:from))

  buy =
    string("B")
    |> replace(:buy)
    |> unwrap_and_tag(:verb)
    |> concat(buyable |> tag(:what))

  place =
    string("P")
    |> replace(:place)
    |> unwrap_and_tag(:verb)
    |> concat(
      choice([
        village |> unwrap_and_tag(:what) |> concat(vertex_coordinate |> tag(:at)),
        city |> unwrap_and_tag(:what) |> concat(vertex_coordinate |> tag(:at)),
        road |> unwrap_and_tag(:what) |> concat(edge_coordinate |> tag(:at))
      ])
    )

  use_development =
    string("U")
    |> replace(:use_development)
    |> unwrap_and_tag(:verb)
    |> concat(
      choice([
        knight |> unwrap_and_tag(:what),
        monopoly |> unwrap_and_tag(:what) |> concat(resource |> unwrap_and_tag(:on)),
        road_building |> unwrap_and_tag(:what),
        year_of_plenty
        |> unwrap_and_tag(:what)
        |> concat(bracketed_resource_fomula |> tag(:for))
      ])
    )

  trade =
    string("T")
    |> replace(:trade)
    |> unwrap_and_tag(:verb)
    |> concat(bracketed_resource_fomula |> tag(:what))
    |> concat(bracketed_resource_fomula |> tag(:for))
    |> concat(player |> tag(:with))

  end_turn = string("E") |> replace(:end_turn) |> unwrap_and_tag(:verb)

  action =
    choice([roll, move_robber, abandon, steal, buy, place, use_development, trade, end_turn])

  def to_map_deep([{k, v} | t]) when is_atom(k) do
    Map.put_new(to_map_deep(t), k, to_map_deep(v))
  end

  def to_map_deep([]) do
    %{}
  end

  def to_map_deep({k, v}) when is_atom(k) do
    %{k => v}
  end

  def to_map_deep(l) when is_list(l) do
    List.to_tuple(l)
  end

  def to_map_deep(x) do
    x
  end

  defparsecp(:parsec_action, action |> eos(), export_metadata: true)

  def parse_action(str) do
    {:ok, result, _, _, _, _} = parsec_action(str)
    {:ok, to_map_deep(result)}
  end
end
