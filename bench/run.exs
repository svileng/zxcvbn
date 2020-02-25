# For benchmarking, because both packages have
# identical names, you need to rename this
# project `ZxcvbnNif`/`:zxcvbn_nif` or similar.

# Benchee.run(
#   %{
#     "zxcvbn-nif" => fn input -> ZxcvbnNif.run(input) end,
#     "zxcvbn-elixir" => fn input -> ZXCVBN.zxcvbn(input) end
#   },
#   inputs: %{
#     "Password: password" => "password",
#     "Password: elixir nifs are great" => "elixir nifs are great"
#   }
# )
