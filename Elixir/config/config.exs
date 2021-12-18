import Config

config :logger,
       :console,
       level: :info,
       format: "$time $metadata[$level] $levelpad$message\n"