defmodule Native do
  @moduledoc false
  # use Rustler, otp_app: :zxcvbn, crate: "zxcvbn_wrapper"

  # # When your NIF is loaded, it will override this function.
  # def run(_, _ \\ []), do: :erlang.nif_error(:nif_not_loaded)
end
