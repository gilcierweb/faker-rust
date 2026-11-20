# faker_rust::world_cup

available since version 1.9.0.

```rust
world_cup::team() //=> "south korea"

world_cup::stadium() //=> "ekaterinburg arena"

world_cup::city() //=> "saint petersburg"

world_cup::group("group_A") //=> "egypt"
world_cup::group("group_A") //=> "saudi arabia"

world_cup::roster("brazil", "goalkeeper") //=> "héctor cúper"
world_cup::roster("brazil", "goalkeeper") //=> "stanislav cherchesov"
```
