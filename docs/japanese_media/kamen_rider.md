# faker_rust::japanese_media::kamen_rider

```rust
kamen_rider::kamen_rider(&["decade"]) //=> "Kamen Rider Revice"
kamen_rider::user() //=> "Ikki Igarashi"
kamen_rider::series() //=> "Kamen Rider Revice"
```

## Passing Eras

If you want Kamen Rider material from a specific era, you can either pass an
argument:

```rust
kamen_rider::series()
=> "Kamen Rider Saber"
```

...or configure which eras you'd like to use on the class:

```rust
kamen_rider::eras = [:heisei, :reiwa]
kamen_rider::series
=> "Kamen Rider Kiva"
```

## Verified Rust API examples

```rust
kamen_rider::eras() //=> ["showa", "heisei", "reiwa"]
```
