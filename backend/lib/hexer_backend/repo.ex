defmodule HexerBackend.Repo do
  use Ecto.Repo,
    otp_app: :hexer_backend,
    adapter: Ecto.Adapters.Postgres
end
