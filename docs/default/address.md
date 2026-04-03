# faker::default::address

generate address data.

```rust
use faker::address;

address::city() //=> "imogeneborough"

address::street_name() //=> "larkin fork"

address::street_address() //=> "282 kevin brook"

address::secondary_address() //=> "apt. 672"

address::building_number() //=> "7304"

address::mail_box() //=> "po box 123"

address::community() //=> "university crossing"

// note: zip_code will return a random string in zipcode format
address::zip_code() //=> "58517" or "23285-4905"

address::zip() //=> "58517" or "66259-8212"

address::postcode() //=> "76032-4907" or "58517"

address::time_zone() //=> "asia/yakutsk"

address::street_suffix() //=> "street"

address::city_suffix() //=> "fort"

address::city_prefix() //=> "lake"

address::state() //=> "california"

address::state_abbr() //=> "ap"

address::country() //=> "french guiana"

address::country_code() //=> "it"

address::country_code_long() //=> "ita"

address::latitude() //=> "-58.17256227443719"

address::longitude() //=> "-156.65548382095133"

address::full_address() //=> "282 kevin brook, imogeneborough, ca 58517"
```
