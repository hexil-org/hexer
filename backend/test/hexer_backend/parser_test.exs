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
             {:ok, %{verb: :steal, what: :unknown, from: %{player_number: 1}}}
  end

  test "parses 'Steal unknown from bank' action (nonsensical but should work)" do
    assert Parser.parse_action("S?0") ==
             {:ok, %{verb: :steal, what: :unknown, from: %{player_number: 0}}}
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
             {:ok, %{verb: :buy, what: %{type: :development, which: :unknown}}}
  end

  test "parses 'Buy knight development' action" do
    assert Parser.parse_action("BDk") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :knight}}}
  end

  test "parses 'Buy 2-road development' action" do
    assert Parser.parse_action("BDk") ==
             {:ok, %{verb: :buy, what: %{type: :development, which: :two_roads}}}
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
end
