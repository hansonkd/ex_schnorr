# ExSchnorr

## Schnorr Signatures for elixir:

```elixir
{:ok, pub, priv} = ExSchnorr.keypair()
{:ok, signature} = ExSchnorr.sign(priv, "Some Message", "Some Context")
{:ok, is_valid} = ExSchnorr.verify(priv, "Some Message", signature, "Some Context")

{:ok, bytes} = ExSchnorr.priv_to_bytes(priv)
{:ok, key} = ExSchnorr.priv_from_bytes(bytes)

{:ok, pubbytes} = ExSchnorr.public_to_bytes(priv)
{:ok, pubkey} = ExSchnorr.public_from_bytes(pubbytes)
```