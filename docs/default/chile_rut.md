# faker_rust::chile_rut

```rust
chilerut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"

chilerut::rut() //=> 11235813

chilerut::rut() //=> 31853211

chilerut::rut() //=> 23567131
chilerut::last_rut() //=> 23567131
chilerut::dv("12345678") //=> "k"

chilerut::rut() //=> 30528772
chilerut::check_digit("12345678") //=> "5"

chilerut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"
chilerut::full_rut(1, 99_999_999, false, true) //=> "20890156-4"
chilerut::full_rut(1, 99_999_999, false, true) //=> "20.890.156-4"
chilerut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"
```

## Verified Rust API examples

`last_rut()` exposes the body of the most recent `rut()` or `full_rut()` call,
mirroring the gem's stateful reader:

```rust
// after a call to chile_rut::rut()
chile_rut::last_rut() //=> Some(6823896)
```
