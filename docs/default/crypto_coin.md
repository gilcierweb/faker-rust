# faker_rust::crypto_coin

```rust
crypto_coin::coin_name() //=> "bitcoin"

crypto_coin::acronym() //=> "btc"

crypto_coin::url_logo() //=> "https://i.imgur.com/efz61ei.png"

crypto_coin::coin_array() //=> ["dash", "dash", "https://i.imgur.com/2ux91cb.png"]

crypto_coin::coin_hash {:name=>"ethereum", :acronym=>"eth", :url_logo=>"https://i.imgur.com/uopfcxj.png"}
```

## Verified Rust API examples

`coin_hash()` returns one coin record with consistent fields:

```rust
crypto_coin::coin_hash() //=> {"name":"Ethereum","acronym":"ETH","url_logo":"https://i.imgur.com/uOPFCXj.png"}
```
