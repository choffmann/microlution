defmodule ElixirSanga.MixProject do
  use Mix.Project

  def project do
    [
      app: :elixir_sanga,
      version: "0.1.0",
      # for compatibility with openflexure raspbian package
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:circuits_uart, ">= 1.5.3"},
      # Lock elixir_make to a version compatible with Elixir 1.7
      {:elixir_make, "~> 0.6.0", override: true}
    ]
  end
end
