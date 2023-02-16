defmodule Backend.Game.Server do
  use GenServer

  # Client

  def start_link(name) do
    GenServer.start_link(__MODULE__, [], name: via_tuple(name))
  end

  # Server (callbacks)

  @impl true
  def init(game) do
    {:ok, game}
  end

  # Private

  defp via_tuple(game_name) do
    # {:via, module_name, term}
    {:via, :gproc, {:n, :l, {:game, game_name}}}
  end
end
