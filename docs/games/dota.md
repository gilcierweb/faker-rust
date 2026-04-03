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

// Random quote
# by default if you don't pass the hero parameter, the quote method will set hero as 'abbadon'
dota::quote() //=> "You have called death upon yourself."
dota::quote() //=> "Better living through alchemy!"
```
