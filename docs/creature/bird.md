# faker_rust::creature::bird

```rust
// Random common family name of a bird
bird::common_family_name() //=> "Owls"

// Random taxonomic order from the class Aves (ie. those are birds)
bird::order() //=> "Passeriformes" # Fun fact, 60% of birds are Passeriformes!

// Random bird anatomy word
bird::anatomy() //=> "rump"

// Random bird anatomy word, in the past tense
bird::anatomy_past_tense() //=> "breasted"

// Random bird geography word
bird::geo() //=> "Eurasian"

// Random bird color word
bird::color() //=> "ferruginous"

// Random bird adjective word
bird::adjective() //=> "common"

// Random emotional adjective NOT typically used in bird names
bird::emotional_adjective() //=> "cantankerous"

// Random silly adjective NOT used in bird names
bird::silly_adjective() //=> "drunk"

// Random common name for a bird
bird::common_name() //=> 'wren'

// Random plausible common name for a bird
bird::plausible_common_name() //=> 'Hellinger's Wren'

// Random implausible common name for a bird
bird::implausible_common_name() //=> 'Hellinger's Cantankerous Chickadee'

// Returns a random pair order / common name pair
bird::order_with_common_name() //=> {:order=>"Coliiformes", :common_name=>"Mousebird"}
```
