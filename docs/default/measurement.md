
# faker::Measurement

### For each, if you don't pass in an integer or float, it randomly selects an integer between 1 and 10.
### If you pass in "none", it returns the singular version of the measurement unit, without a number.
### If you pass in "all", it returns the plural version of the measurement unit, without a number.

```rust
# Keyword arguments: amount
Measurement::height() //=> "6 inches"
Measurement:;height(amount: 1.4) #=> "1.4 inches"
Measurement:;height(amount: "none") #=> "inch"
Measurement:;height(amount: "all") #=> "inches"

Measurement::length() //=> "1 yard"

Measurement::volume() //=> "10 cups"

Measurement::weight() //=> "3 pounds"

Measurement::metric_height() //=> "2 meters"

Measurement::metric_length() //=> "0 decimeters"

Measurement::metric_volume() //=> "1 liter"

Measurement::metric_weight() //=> "8 grams"
```
