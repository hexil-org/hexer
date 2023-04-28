defmodule HexerBackend.ParserTest do
  use ExUnit.Case
  alias HexerBackend.Parser

  test "parses 'Roll 2+3' action" do
    assert Parser.action("R(2+3)") == %{type: :roll, what: {2, 3}}
  end
end
