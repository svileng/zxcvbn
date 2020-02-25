# zxcvbn for Elixir (Rust NIF) [![hex.pm](https://img.shields.io/hexpm/v/zxcvbn_nif.svg?style=flat-square)](https://hex.pm/packages/zxcvbn_nif) [![hexdocs.pm](https://img.shields.io/badge/docs-latest-green.svg?style=flat-square)](https://hexdocs.pm/zxcvbn_nif)

This library is based on the [zxcvbn-rs](https://github.com/shssoichiro/zxcvbn-rs) package written in Rust. It is ported to Elixir as a NIF and offers significantly improved performance over other Elixir libraries.

For more general information, see the original [zxcvbn](https://github.com/dropbox/zxcvbn) package by Dropbox.

## Installation

The package is [available in Hex](https://hex.pm/packages/zxcvbn_nif) and can be installed
by adding `zxcvbn_nif` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:zxcvbn_nif, "~> 1.0"}
  ]
end
```

## Usage

Full documentation is published here: [https://hexdocs.pm/zxcvbn_nif](https://hexdocs.pm/zxcvbn_nif).

Just call `Zxcvbn.run/1` with a password to analyse:

```elixir
iex> Zxcvbn.run("password")
{:ok,
 %{
   calc_time: 1.0761590003967285,
   crack_times_display: %{
     offline_fast_hashing_1e10_per_second: "less than a second",
     offline_slow_hashing_1e4_per_second: "less than a second",
     online_no_throttling_10_per_second: "less than a second",
     online_throttling_100_per_hour: "1 minute"
   },
   feedback: %{
     suggestions: ["Add another word or two. Uncommon words are better.",
      "Reversed words aren't much harder to guess."],
     warning: "This is similar to a commonly used password."
   },
   guesses: 3,
   guesses_log10: 0.47712125471966244,
   score: 0
 }}
```

You can also supply a second optional parameter, to use your own word dictionary:

```elixir
iex> Zxcvbn.run("konnibanwa", ["konni", "banwa"])
{:ok,
 %{
   calc_time: 0.6378800272941589,
   crack_times_display: %{
     offline_fast_hashing_1e10_per_second: "less than a second",
     offline_slow_hashing_1e4_per_second: "1 second",
     online_no_throttling_10_per_second: "25 minutes",
     online_throttling_100_per_hour: "6 days"
   },
   feedback: %{
     suggestions: ["Add another word or two. Uncommon words are better."],
     warning: nil
   },
   guesses: 15000,
   guesses_log10: 4.176091259055681,
   score: 1
 }}
```

To compare, here is the result without using a custom dictionary:

```elixir
iex> Zxcvbn.run("konnibanwa")
{:ok,
 %{
   calc_time: 0.49091002345085144,
   crack_times_display: %{
     offline_fast_hashing_1e10_per_second: "less than a second",
     offline_slow_hashing_1e4_per_second: "10 days",
     online_no_throttling_10_per_second: "29 years",
     online_throttling_100_per_hour: "centuries"
   },
   feedback: %{suggestions: nil, warning: nil},
   guesses: 9460000000,
   guesses_log10: 9.975891136401792,
   score: 3
 }}
```

## Benchmarks
Compared to [zxcvbn](https://hex.pm/packages/zxcvbn) (pure Elixir) package:

```
Operating System: macOS
CPU Information: Intel(R) Core(TM) i7-5557U CPU @ 3.10GHz
Number of Available Cores: 4
Available memory: 16 GB
Elixir 1.9.2
Erlang 21.0.3

##### With input Password: elixir nifs are great #####
Name                    ips        average  deviation         median         99th %
zxcvbn-nif           785.01        1.27 ms    ±26.05%        1.21 ms        2.42 ms
zxcvbn-elixir         37.51       26.66 ms    ±15.42%       25.99 ms       50.74 ms

Comparison:
zxcvbn-nif           785.01
zxcvbn-elixir         37.51 - 20.93x slower +25.38 ms

##### With input Password: password #####
Name                    ips        average  deviation         median         99th %
zxcvbn-nif           3.51 K        0.29 ms    ±68.93%        0.27 ms        0.43 ms
zxcvbn-elixir      0.0397 K       25.22 ms    ±12.50%       24.72 ms       43.75 ms

Comparison:
zxcvbn-nif           3.51 K
zxcvbn-elixir      0.0397 K - 88.44x slower +24.93 ms
```

## License

See LICENSE.md file.
