# faker_rust::games::dota

Available since version 1.9.0.

```rust
// Random hero
dota::hero() //=> "Abaddon"

// Random item
dota::item() //=> "Armlet of Mordiggian"

// Random team
dota::team() //=> "Evil Geniuses"

// Random player
dota::player() //=> "Dendi"

// Random quote for a given hero. Rust requires the hero name explicitly.
dota::quote("abaddon") //=> "You have called death upon yourself."
dota::quote("abaddon") //=> "Better living through alchemy!"
```
