defmodule HexerBackend.ParserTest do
  use ExUnit.Case
  alias HexerBackend.Parser

  test "parses 'Roll 2+3' action" do
    assert Parser.action("R(2+3)") == %{verb: :roll, what: {2, 3}}
  end

  test "parses 'Steal' action" do
    assert Parser.action("SO2") == %{verb: :steal, what: :ore, from: 2}
  end

  test "parses 'Steal' action with unknown resource" do
    assert Parser.action("S?1") == %{verb: :steal, what: :unknown, from: 1}
  end

  test "parses 'Abandon' action" do
    assert Parser.action("2A(O2)") == %{who: 2, verb: :abandon, what: %{ore: 2}}
  end
end
