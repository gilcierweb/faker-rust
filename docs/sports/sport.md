# faker_rust::sports::sport

// Sport

## Conventional (Olympic including Paralympic) Sports

```rust
// Any one of the below four categories:
sport::sport() //=> "Snowboard"

sport::summer_olympics_sport() //=> "Triathlon"

sport::winter_olympics_sport() //=> "Luge"

sport::summer_paralympics_sport() //=> "Goalball"

sport::winter_paralympics_sport() //=> "Wheelchair curling"
```

## Ancient Sports

```rust
sport::ancient_olympics_sport() //=> "Chariot racing"

// Any modern or ancient olympic sport:
sport::sport() //=> "Rugby"
```

## Unusual Sports (just for fun)

```rust
sport::unusual_sport() //=> "Camel wrestling"

// Any modern olympic or unusual sport:
sport::sport() //=> "Gurning"
```

## Full list

```rust
// Modern, ancient or unusual:
sport::sport() //=> "Powerlifting"
```
