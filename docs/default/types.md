# faker::default::types

```rust
types::rb_string() //=> "foobar"

types::character() //=> "n"

types::rb_integer() //=> 1

types::rb_hash() //=> {name: "bob"}
types::rb_hash() //=> {name: "bob"}
types::rb_hash() //=> {name: "bob", last: "marley"}

types::complex_rb_hash() //=> {user: {first: "bob", last: "marley"}}
types::complex_rb_hash() //=> {user: {first: "bob", last: "marley"}}
types::complex_rb_hash() //=> {user: {first: "bob", last: "marley"}, son: ["damien", "marley"]}

types::rb_array() //=> ["a"]
types::rb_array() //=> ["a", 1, 2, "bob"]
types::rb_array() //=> ["cat", "foo"]

types::random_type() //=> 1 or "a" or "bob"

types::random_complex_type() //=> 1 or "a" or "bob" or {foo: "bar"}
```
