# faker_rust::types

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

## Verified Rust API examples

Real outputs observed from the crate:

```rust
types::character() //=> "w"
types::complex_rb_hash(2) //=> Object {"tenetur": Array [Number(41)], "natus": Array [Number(97)]}
types::random_complex_type() //=> Object {"veritatis": Number(37)}
types::random_type() //=> String("molestiae")
types::rb_array(3) //=> Array [Number(9), String("provident"), Number(84)]
types::rb_hash(2) //=> Object {"labore": String("et"), "et": Number(82)}
types::rb_integer(1, 100) //=> 73
types::rb_string(3) //=> "et aut sit"
```
