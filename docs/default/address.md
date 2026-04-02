# faker::address

Generate address data.

```rust
use faker::address;

address::city() //=> "Imogeneborough"

address::street_name() //=> "Larkin Fork"

address::street_address() //=> "282 Kevin Brook"

address::secondary_address() //=> "Apt. 672"

address::building_number() //=> "7304"

address::mail_box() //=> "PO Box 123"

address::community() //=> "University Crossing"

// Note: zip_code will return a random string in zipcode format
address::zip_code() //=> "58517" or "23285-4905"

address::zip() //=> "58517" or "66259-8212"

address::postcode() //=> "76032-4907" or "58517"

address::time_zone() //=> "Asia/Yakutsk"

address::street_suffix() //=> "Street"

address::city_suffix() //=> "fort"

address::city_prefix() //=> "Lake"

address::state() //=> "California"

address::state_abbr() //=> "AP"

address::country() //=> "French Guiana"

address::country_code() //=> "IT"

address::country_code_long() //=> "ITA"

address::latitude() //=> "-58.17256227443719"

address::longitude() //=> "-156.65548382095133"

address::full_address() //=> "282 Kevin Brook, Imogeneborough, CA 58517"
```
