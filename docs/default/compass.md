
# faker::Compass

Available since version 1.8.0.

```rust
# A random direction
Compass.direction                 #=> "southeast"

# Random cardinal
Compass.cardinal                  #=> "north"

# Random ordinal
Compass.ordinal                   #=> "northwest"

# Random half_wind
Compass.half_wind                 #=> "north-northwest"

# Random quarter wind
Compass.quarter_wind              #=> "north by west"

# Random abbreviation
Compass.abbreviation              #=> "NEbN"

# Random cardinal abbreviation
Compass.cardinal_abbreviation     #=>    "N"

# Random ordinal abbreviation
Compass.ordinal_abbreviation      #=>   "SW"

# Random half wind abbreviation
Compass.half_wind_abbreviation    #=>  "NNE"

# Random quarter wind abbreviation
Compass::quarter_wind_abbreviation() //=> "SWbS"

# Random azimuth
Compass.azimuth                   #=> "168.75"

# Random cardinal azimuth
Compass.cardinal_azimuth          #=>     "90"

# Random ordinal azimuth
Compass.ordinal_azimuth           #=>    "135"

# Random half wind azimuth
Compass.half_wind_azimuth         #=>  "292.5"

# Random quarter wind azimuth
Compass.quarter_wind_azimuth      #=>  "56.25"
```
