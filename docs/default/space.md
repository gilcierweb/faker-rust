
# faker::Space

Available since version 1.6.4.

```rust
# Random planet from our Solar System
Space::planet() //=> "Venus"

# Random moon from our Solar System
Space::moon() //=> "Europa"

# Random galaxy
Space::galaxy() //=> "Andromeda"

# Random nebula name
Space::nebula() //=> "Triffid Nebula"

# Random star cluster
Space::star_cluster() //=> "Messier 70"

# Random constellation
Space::constellation() //=> "Orion"

# Random star
Space::star() //=> "Proxima Centauri"

# Random national space agency
Space::agency() //=> "Japan Aerospace Exploration Agency"

# Random space agency abbreviation
Space::agency_abv() //=> "NASA"

# Random spacecraft name (limited to NASA)
Space::nasa_space_craft() //=> "Endeavour"

# Random private space company title
Space::company() //=> "SpaceX"

# Random unit of stellar distance with number
Space::distance_measurement() //=> "15 parsecs"

# Random meteorite name
Space::meteorite() //=> "Ensisheim"

# Random launch vehicle name
Space::launch_vehicle() //=> "Saturn IV"
```
