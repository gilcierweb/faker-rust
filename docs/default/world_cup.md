
# faker::WorldCup

Available since version 1.9.0.

```rust
WorldCup::team() //=> "South Korea"

WorldCup::stadium() //=> "Ekaterinburg Arena"

WorldCup::city() //=> "Saint Petersburg"

# Keyword arguments: group
WorldCup::group() //=> "Egypt"
WorldCup:;group(group: 'group_A') #=> "Saudi Arabia"

# Keyword arguments: country, type
WorldCup::roster() //=> "Héctor Cúper"
WorldCup:;roster(country: 'Russia', type: 'coach') #=> "Stanislav Cherchesov"
```
