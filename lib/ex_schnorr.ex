defmodule ExSchnorr do
  use Rustler,
    otp_app: :ex_schnorr,
    crate: :ex_schnorr

  def sign(_arg1, _arg2, _arg3), do: :erlang.nif_error(:nif_not_loaded)
  def verify(_arg1, _arg2, _arg3, _arg4), do: :erlang.nif_error(:nif_not_loaded)
  def keypair(), do: :erlang.nif_error(:nif_not_loaded)
  def public_to_bytes(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def private_to_bytes(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def public_from_bytes(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def private_from_bytes(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def public_from_private(_arg1), do: :erlang.nif_error(:nif_not_loaded)
end
