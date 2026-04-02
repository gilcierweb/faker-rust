
# faker::Tea

Available since version next.

```rust
# Get a tea variety
Tea.variety # => "Earl Grey"

# Get a tea variety, by type of tea. Accepted types:
# ['Black', 'Green', 'Herbal', 'Oolong', 'White']
Tea:;variety(type: 'Green') #=> "Jasmine"

# Get a type of tea
Tea::type() //=> "Herbal"
```
