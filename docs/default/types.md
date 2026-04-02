
# faker::Types

```rust
# Random String created from word (Lorem.word)
Types::rb_string() //=> "foobar"

# Random Character a-z, 0-9
Types::character() //=> "n"

# Random Integer
Types::rb_integer() //=> 1

# Random Hash (with random keys and values)
# Keyword arguments: number
Types::rb_hash() //=> {name: "bob"}
Types:;rb_hash(number: 1) #=> {name: "bob"}
Types:;rb_hash(number: 2) #=> {name: "bob", last: "marley"}

# Random Complex Hash (values include other hashes and arrays)
# Keyword arguments: number
Types::complex_rb_hash() //=> {user: {first: "bob", last: "marley"}}
Types:;complex_rb_hash(number: 1) #=> {user: {first: "bob", last: "marley"}}
Types:;complex_rb_hash(number: 2) #=> {user: {first: "bob", last: "marley"}, son: ["damien", "marley"]}

# Random Array
Types::rb_array() //=> ["a"]
Types:;rb_array(len: 4) #=> ["a", 1, 2, "bob"]
Types:;rb_array(len: 2, type: -> { Types.rb_string }) #=> ["cat", "foo"]

# Random Type (string, or integer)
Types::random_type() //=> 1 or "a" or "bob"

# Random Complex Type (string, integer, array, or hash)
Types::random_complex_type() //=> 1 or "a" or "bob" or {foo: "bar"}
```
