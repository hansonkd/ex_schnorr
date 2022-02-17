defmodule ExSchnorr.MixProject do
  use Mix.Project

  @version "0.1.0"
  def project do
    [
      app: :ex_schnorr,
      version: @version,
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      package: package(),
      source_url: "https://github.com/hansonkd/ex_schnorr"
    ]
  end

  defp docs() do
    [
      extras: [
        LICENSE: [title: "License"],
        "README.md": [title: "Overview"]
      ],
      main: "readme",
      assets: "assets",
      canonical: "http://hexdocs.pm/sorted_set_kv",
      source_url: "https://github.com/hansonkd/ex_schnorr",
      source_ref: "v#{@version}",
      formatters: ["html"]
    ]
  end

  defp package() do
    [
      description: "Cryptographic Schnorr Signatures for Elixir written in Rust",
      files: [
        "lib",
        "native/ex_schnorr/src",
        "native/ex_schnorr/Cargo.toml",
        "LICENSE",
        "mix.exs"
      ],
      maintainers: ["Kyle Hanson"],
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/hansonkd/ex_schnorr"
      }
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
      {:rustler, "~> 0.23.0"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
