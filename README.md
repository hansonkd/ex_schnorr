
## Schnorr Signatures for Elixir:

Cryptographic Schnorr signatures in Elixir

# ExSchnorr

```elixir
{:ok, pub, priv} = ExSchnorr.keypair()
{:ok, signature} = ExSchnorr.sign(priv, "Some Message", "Some Context")
{:ok, is_valid} = ExSchnorr.verify(pub, "Some Message", signature, "Some Context")

{:ok, bytes} = ExSchnorr.priv_to_bytes(priv)
{:ok, key} = ExSchnorr.priv_from_bytes(bytes)

{:ok, pubbytes} = ExSchnorr.public_to_bytes(pub)
{:ok, pubkey} = ExSchnorr.public_from_bytes(pubbytes)

{:ok, pubkey2} = ExSchnorr.public_from_private(priv)
```
