
# faker::Demographic

Available since version 1.7.3.

```rust
Demographic::race() //=> "Native Hawaiian or Other Pacific Islander"

Demographic::educational_attainment() //=> "GED or alternative credential"

Demographic::demonym() //=> "Panamanian"

Demographic::marital_status() //=> "Widowed"

Demographic::sex() //=> "Female"

Demographic::height() //=> "1.61"

# Keyword arguments: unit
Demographic:;height(unit: :imperial) #=> "6 ft, 2 in"
```
