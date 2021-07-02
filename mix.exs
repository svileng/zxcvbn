defmodule Zxcvbn.MixProject do
  use Mix.Project

  @version "1.0.4"

  def project do
    [
      app: :zxcvbn_nif,
      version: @version,
      elixir: "~> 1.9",
      name: "Zxcvbn",
      description: "zxcvbn for Elixir using Rust NIF.",
      deps: [
        {:rustler, "~> 0.22.0"},
        {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
        {:benchee, "~> 1.0", only: :dev}
        # {:zxcvbn, "~> 0.1.3", only: :dev}
      ],
      package: [
        name: "zxcvbn_nif",
        maintainers: ["Svilen Gospodinov <webmaster@s2g.io>"],
        licenses: ["MIT"],
        links: %{Github: "https://github.com/svileng/zxcvbn"},
        files: ["lib", "native", "mix.exs", "README.md", "LICENSE.md"]
      ],
      docs: [
        main: "zxcvbn",
        canonical: "http://hexdocs.pm/zxcvbn",
        source_url: "https://github.com/svileng/zxcvbn",
        source_ref: @version
      ]
    ]
  end
end
