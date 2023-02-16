import Config

config :backend, Backend.UserSocket,
  port: 8080,
  path: "/",
  max_connections: 10000,
  max_connection_age: :infinity,
  idle_timeout: 120_000,
  reuse_port: false,
  show_debug_logs: false,
  transmission_limit: [
    # if 50 frames are sent on a connection
    capacity: 50,
    # in 2 seconds, disconnect it.
    duration: 2000
  ],
  cowboy_opts: [
    # ...
  ]

import_config "#{config_env()}.exs"
