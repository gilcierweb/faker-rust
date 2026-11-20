# faker_rust::chile_rut

```rust
chile_rut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"

chile_rut::rut() //=> 11235813

chile_rut::rut() //=> 31853211

chile_rut::rut() //=> 23567131
chile_rut::last_rut() //=> 23567131
chile_rut::dv("12345678") //=> "k"

chile_rut::rut() //=> 30528772
chile_rut::check_digit("12345678") //=> "5"

chile_rut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"
chile_rut::full_rut(1, 99_999_999, false, true) //=> "20890156-4"
chile_rut::full_rut(1, 99_999_999, false, true) //=> "20.890.156-4"
chile_rut::full_rut(1, 99_999_999, false, true) //=> "30686957-4"
```

## Verified Rust API examples

`last_rut()` exposes the body of the most recent `rut()` or `full_rut()` call,
mirroring the gem's stateful reader:

```rust
// after a call to chile_rut::rut()
chile_rut::last_rut() //=> Some(6823896)
```
