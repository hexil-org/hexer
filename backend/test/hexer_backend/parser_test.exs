defmodule HexerBackend.ParserTest do
  use ExUnit.Case
  alias HexerBackend.Parser

  test "parses 'Roll 2+3' action" do
    assert Parser.parse_action("R(2+3)") == {:ok, %{verb: :roll, what: {2, 3}}}
  end

  test "parses 'Steal' action" do
    assert Parser.parse_action("SO2") == {:ok, %{verb: :steal, what: :ore, from: 2}}
  end

  test "parses 'Steal' action with unknown resource" do
    assert Parser.parse_action("S?1") == {:ok, %{verb: :steal, what: :unknown, from: 1}}
  end

  test "parses 'Abandon' action" do
    assert Parser.parse_action("2A(O2)") == {:ok, %{who: 2, verb: :abandon, what: %{ore: 2}}}
  end
end
