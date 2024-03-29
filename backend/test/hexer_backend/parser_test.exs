defmodule HexerBackend.ParserTest do
  use ExUnit.Case
  alias HexerBackend.Parser

  test "parses 'Roll 2+3' action" do
    assert Parser.parse_action("R(2+3)") == {:ok, %{verb: :roll, what: %{low: 2, high: 3}}}
  end

  test "parses 'Roll 3+2' action" do
    assert Parser.parse_action("R(3+2)") == {:ok, %{verb: :roll, what: %{low: 2, high: 3}}}
  end

  test "parses 'Move Robber' action" do
    assert Parser.parse_action("Md4") == {:ok, %{verb: :move_robber, to: %{q: 4, r: 4}}}
  end

  test "parses 'Move Robber' action with big coordinate" do
    assert Parser.parse_action("Maa180") == {:ok, %{verb: :move_robber, to: %{q: 27, r: 180}}}
  end

  test "parses 'Abandon' action" do
    assert Parser.parse_action("2A(W)") ==
             {:ok, %{who: %{player_number: 2}, verb: :abandon, what: %{wool: 1}}}
  end

  test "parses 'Abandon' action with bigger formula" do
    assert Parser.parse_action("2A(BO2W5)") ==
             {:ok,
              %{who: %{player_number: 2}, verb: :abandon, what: %{brick: 1, ore: 2, wool: 5}}}
  end

  test "parses 'Steal' action" do
    assert Parser.parse_action("SO2") ==
             {:ok, %{verb: :steal, what: :ore, from: %{player_number: 2}}}
  end

  test "parses 'Steal' action with unknown resource" do
    assert Parser.parse_action("S?1") ==
             {:ok, %{verb: :steal, what: :unknown_resource, from: %{player_number: 1}}}
  end

  test "parses 'Steal unknown from bank' action (nonsensical but should work)" do
    assert Parser.parse_action("S?0") ==
             {:ok, %{verb: :steal, what: :unknown_resource, from: %{player_number: 0}}}
  end

  test "parses 'Buy village' action" do
    assert Parser.parse_action("BV") ==
             {:ok, %{verb: :buy, what: %{type: :structure, which: :village}}}
  end

  test "parses 'Buy city' action" do
    assert Parser.parse_action("BC") ==
             {:ok, %{verb: :buy, what: %{type: :structure, which: :city}}}
  end

  test "parses 'Buy road' action" do
    assert Parser.parse_action("BR") ==
             {:ok, %{verb: :buy, what: %{type: :structure, which: :road}}}
  end

  test "parses 'Buy unknown development' action" do
    assert Parser.parse_action("BD?") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :unknown_development}}}
  end

  test "parses 'Buy knight development' action" do
    assert Parser.parse_action("BDk") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :knight}}}
  end

  test "parses 'Buy 2-road development' action" do
    assert Parser.parse_action("BDr") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :road_building}}}
  end

  test "parses 'Buy monopoly development' action" do
    assert Parser.parse_action("BDm") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :monopoly}}}
  end

  test "parses 'Buy victory point development' action" do
    assert Parser.parse_action("BDv") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :victory_point}}}
  end

  test "parses 'Buy year of plenty development' action" do
    assert Parser.parse_action("BDy") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :year_of_plenty}}}
  end

  test "parses 'Place village' action" do
    assert Parser.parse_action("PVc4n") ==
             {:ok, %{verb: :place, what: :village, at: %{q: 3, r: 4, corner: :north}}}
  end

  test "parses 'Place city' action" do
    assert Parser.parse_action("PCc4s") ==
             {:ok, %{verb: :place, what: :city, at: %{q: 3, r: 4, corner: :south}}}
  end

  test "parses 'Place road on west border' action" do
    assert Parser.parse_action("PRc4w") ==
             {:ok, %{verb: :place, what: :road, at: %{q: 3, r: 4, border: :west}}}
  end

  test "parses 'Place road on north-east border' action" do
    assert Parser.parse_action("PRc4ne") ==
             {:ok, %{verb: :place, what: :road, at: %{q: 3, r: 4, border: :north_east}}}
  end

  test "parses 'Place road on north-west border' action" do
    assert Parser.parse_action("PRc4nw") ==
             {:ok, %{verb: :place, what: :road, at: %{q: 3, r: 4, border: :north_west}}}
  end

  test "parses 'Use knight development' action" do
    assert Parser.parse_action("UDk") == {:ok, %{verb: :use_development, what: :knight}}
  end

  test "parses 'Use monopoly development' action" do
    assert Parser.parse_action("UDmL") ==
             {:ok, %{verb: :use_development, what: :monopoly, on: :lumber}}
  end

  test "parses 'Use road building development' action" do
    assert Parser.parse_action("UDr") == {:ok, %{verb: :use_development, what: :road_building}}
  end

  test "parses 'Use year of plenty development' action" do
    assert Parser.parse_action("UDy(LO)") ==
             {:ok, %{verb: :use_development, what: :year_of_plenty, for: %{lumber: 1, ore: 1}}}
  end

  test "parses 'Trade' action" do
    assert Parser.parse_action("T(LO2)(B3)1") ==
             {:ok,
              %{
                verb: :trade,
                what: %{lumber: 1, ore: 2},
                for: %{brick: 3},
                with: %{player_number: 1}
              }}
  end

  test "Parses 'End turn' action" do
    assert Parser.parse_action("E") == {:ok, %{verb: :end_turn}}
  end
end
