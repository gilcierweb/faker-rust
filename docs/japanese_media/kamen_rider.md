# faker::japanese_media::KamenRider

```rust
KamenRider::kamen_rider() //=> "Kamen Rider Revice"
KamenRider::user() //=> "Ikki Igarashi"
KamenRider::series() //=> "Kamen Rider Revice"
```

## Passing Eras

If you want Kamen Rider material from a specific era, you can either pass an
argument:

```rust
KamenRider::series(:reiwa)
=> "Kamen Rider Saber"
```

...or configure which eras you'd like to use on the class:

```rust
KamenRider::eras = [:heisei, :reiwa]
KamenRider::series
=> "Kamen Rider Kiva"
```
