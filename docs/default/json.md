# faker_rust::json

**shallow_json()** -> json formatted string

 takes a width_int and options_hash where the number of key value pairs in the
 returned json string is equal to the width_int.

 `options_hash = {key: class::method, value: class::method}` where class is
 any class in the faker gem.  for example if you want random names for keys
 and random star wars quotes for values you would write the options_hash as follows
 ```ruby
 options_hash = { key: 'name::first_name', value: 'starwars::quote' }
 ```

 it is important to note that you do not need to append your faker gem class calls
 with ``

```rust
json::shallow_json(2) //=> "{\"parliament funkadelic\":\"they're real, and they're spectacular.\",\"fleetwood mac\":\"i'm not a lesbian. i hate men, but i'm not a lesbian.\",\"the roots\":\"it became very clear to me sitting out there today that every decision i've made in my entire life has been wrong...\"}"
```

**add_depth_to_json()** -> json

functions exactly as `json#shallow_json()` except it takes in a json as an
additional argument and returns that json with new generated nested jsons in
place of the lowest nested values.  it is important to note that the json must
be a json and not a hash.

```ruby
json = json::shallow_json(2)
puts json //
{"alisha":"olson","everardo":"dubuque","bridgette":"turner"}

json2 = json::add_depth_to_json("{}", 2)
puts json2 //
{"alisha":{"daisy":"trantow","oda":"haag"},
 "everardo":{"javier":"marvin","eliseo":"schuppe"},
 "bridgette":{"jorge":"kertzmann","lelah":"macgyver"}}

 json3 = json::add_depth_to_json("{}", 2)
 puts json3 //
 {"alisha":
   {"daisy":
     {"bulah":"wunsch","cristian":"champlin","lester":"bartoletti","greg":"jacobson"},
    "oda":
      {"salvatore":"kuhlman","aubree":"okuneva","larry":"schmitt","velva":"gibson"}},
  "everardo":
    {"javier":
      {"eduardo":"orn","laila":"kub","thad":"legros","dion":"wilderman"},
    "eliseo":
      {"olin":"hilpert","marisa":"greenfelder","karlee":"schmitt","judd":"larkin"}},
  "bridgette":
    {"jorge":
      {"eloy":"pfeffer","kody":"hansen","paxton":"lubowitz","abe":"lesch"},
    "lelah":
      {"rick":"wiza","bonita":"bayer","gardner":"auer","felicity":"abbott"}}}
```

## Verified Rust API examples

Real outputs observed from the crate:

```rust
json::add_depth_to_json("{}", 2) //=> "{\"a\":{\"Season\":\"Magaret\",\"Benita\":\"Sandy\"}}"
json::shallow_json(2) //=> "{\"Frankie\":\"Nickole\",\"Maribeth\":\"Chrystal\"}"
```
