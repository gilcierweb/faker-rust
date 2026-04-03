# faker_rust::games::Dota

Available since version 1.9.0.

```rust
// Random hero
Dota::hero() //=> "Abaddon"

// Random item
Dota::item() //=> "Armlet of Mordiggian"

// Random team
Dota::team() //=> "Evil Geniuses"

// Random player
Dota::player() //=> "Dendi"

// Random quote
# by default if you don't pass the hero parameter, the quote method will set hero as 'abbadon'
Dota::quote() //=> "You have called death upon yourself."
Dota::quote() //=> "Better living through alchemy!"
```
