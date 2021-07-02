defmodule Zxcvbn do
  @moduledoc """
  To use `Zxcvbn` simply call `run/1` with a password string to analyse:

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

  The result will be either `{:ok, result}`, `{:error, "blank_password"}` or `{:error, "duration_out_of_range"}`.

  The keys found in a successful response are documented in the [original `zxcvbn` repository](https://github.com/dropbox/zxcvbn) by Dropbox. Note that not all keys from the original library are implemented in `zxcvbn_nif`.

  A second optional parameter `user_inputs` is supported for custom word dictionaries, for example:

      iex> Zxcvbn.run("konnibanwa", ["konni", "banwa"])

  As described in the original `zxcvbn` documentation, the custom dictionaries should be a list of strings.
  """
  use Rustler, otp_app: :zxcvbn_nif, crate: "zxcvbn_wrapper"

  defp run_nif(_, _), do: :erlang.nif_error(:nif_not_loaded)

  @spec run(String.t()) :: {:ok, map} | {:error, String.t()}
  def run(""), do: {:error, "blank_password"}
  def run(password), do: run_nif(password, [])

  @spec run(String.t(), list(String.t())) :: {:ok, map} | {:error, String.t()}
  def run("", _), do: {:error, "blank_password"}

  def run(password, user_input) when is_list(user_input),
    do: run_nif(password, user_input)
end
