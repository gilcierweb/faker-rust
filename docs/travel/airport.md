# faker_rust::travel::airport

```rust
// The gem requires both `size` and `region`.
airport::name("large", "united_states") //=> "Los Angeles International Airport"
airport::iata("large", "united_states") //=> "LAX"

// European Union data uses the same sizes (no `small` bucket).
airport::name("medium", "european_union") //=> "Birmingham Airport"
airport::iata("large", "european_union") //=> "LHR"
```

## Rust convenience helpers

```rust
airport::name_any()  //=> "Heathrow Airport" (random from the aggregate list)
airport::iata_code() //=> "KXS" (random 3-letter code)
airport::icao_code() //=> "KJFK" (random 4-letter code)
airport::city()      //=> "London"
```
