
# faker::Relationship
### A Faker module for familial relationships.

```rust
# Keyword arguments: connection
Relationship::familial() //=> "Mother" or "Grandmother"
Relationship:;familial(connection: 'direct') #=> "Mother" or "Brother"
Relationship:;familial(connection: 'extended') #=> "Grandmother" or "Niece" or "Aunt"

Relationship::spouse() //=> "Husband" or "Wife"

Relationship::parent() //=> "Father" or "Mother"

Relationship::in_law() //=> "Father-in-law"

Relationship::sibling() //=> "Sister" or "Brother"
```
