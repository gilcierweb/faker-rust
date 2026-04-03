# faker_rust::creature::Bird

```rust
// Random common family name of a bird
Bird::common_family_name() //=> "Owls"

// Random taxonomic order from the class Aves (ie. those are birds)
Bird::order() //=> "Passeriformes" # Fun fact, 60% of birds are Passeriformes!

// Random bird anatomy word
Bird::anatomy() //=> "rump"

// Random bird anatomy word, in the past tense
Bird::anatomy_past_tense() //=> "breasted"

// Random bird geography word
Bird::geo() //=> "Eurasian"

// Random bird color word
Bird::color() //=> "ferruginous"

// Random bird adjective word
Bird::adjective() //=> "common"

// Random emotional adjective NOT typically used in bird names
Bird::emotional_adjective() //=> "cantankerous"

// Random silly adjective NOT used in bird names
Bird::silly_adjective() //=> "drunk"

// Random common name for a bird
Bird::common_name() //=> 'wren'

// Random plausible common name for a bird
Bird::plausible_common_name() //=> 'Hellinger's Wren'

// Random implausible common name for a bird
Bird::implausible_common_name() //=> 'Hellinger's Cantankerous Chickadee'

// Returns a random pair order / common name pair
Bird::order_with_common_name() //=> {:order=>"Coliiformes", :common_name=>"Mousebird"}
```
