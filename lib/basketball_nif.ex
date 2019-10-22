defmodule BasketballNIF do
  use Rustler,
    otp_app: :basketball_nif,
    crate: "basketball_nif"

  def new, do: :erlang.nif_error(:nif_not_loaded)
  def update(_state, _action), do: :erlang.nif_error(:nif_not_loaded)
  def get_score(_state), do: :erlang.nif_error(:nif_not_loaded)

end
