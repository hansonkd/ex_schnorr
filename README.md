
## Schnorr Signatures for elixir:

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ex_schnorr` to your list of dependencies in `mix.exs`:

# ExSchnorr

```elixir
{:ok, pub, priv} = ExSchnorr.keypair()
{:ok, signature} = ExSchnorr.sign(priv, "Some Message", "Some Context")
{:ok, is_valid} = ExSchnorr.verify(priv, "Some Message", signature, "Some Context")

{:ok, bytes} = ExSchnorr.priv_to_bytes(priv)
{:ok, key} = ExSchnorr.priv_from_bytes(bytes)

{:ok, pubbytes} = ExSchnorr.public_to_bytes(priv)
{:ok, pubkey} = ExSchnorr.public_from_bytes(pubbytes)
```
