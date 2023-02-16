defmodule Backend.UserSocket do
  use Riverside, otp_app: :backend

  @impl Riverside
  def handle_message(%{"t" => "join", "game" => game_name}, session, state) do
    # TODO: implement
    deliver_me(%{"t" => "joined", "game" => game_name})
    {:ok, session, state}
  end

  @impl Riverside
  def handle_message(msg, session, state) do
    deliver_me(msg)
    {:ok, session, state}
  end
end
