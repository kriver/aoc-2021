defmodule AdventOfCode2021.MixProject do
  use Mix.Project

  def project do
    [
      app: :exlixir_new,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:math, "~> 0.7.0"},
      {:matrex, "~> 0.6"}
    ]
  end
end
