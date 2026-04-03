# faker::sports::sport

// Sport

## Conventional (Olympic including Paralympic) Sports

```rust
// Any one of the below four categories:
Sport::sport() //=> "Snowboard"

Sport::summer_olympics_sport() //=> "Triathlon"

Sport::winter_olympics_sport() //=> "Luge"

Sport::summer_paralympics_sport() //=> "Goalball"

Sport::winter_paralympics_sport() //=> "Wheelchair curling"
```

## Ancient Sports

```rust
Sport::ancient_olympics_sport() //=> "Chariot racing"

// Any modern or ancient olympic sport:
Sport::sport() //=> "Rugby"
```

## Unusual Sports (just for fun)

```rust
Sport::unusual_sport() //=> "Camel wrestling"

// Any modern olympic or unusual sport:
Sport::sport() //=> "Gurning"
```

## Full list

```rust
// Modern, ancient or unusual:
Sport::sport() //=> "Powerlifting"
```
