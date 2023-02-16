defmodule PlayerSocketHandler do
  use Riverside, otp_app: :backend

  @impl Riverside
  def handle_message(msg, session, state) do
    deliver_me(msg)
    {:ok, session, state}
  end
end
