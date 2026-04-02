
# faker::Commerce

```rust
Commerce::color() //=> "lavender"

# Keyword arguments: max, fixed_amount
Commerce::department() //=> "Grocery, Health & Beauty"
Commerce:;department(max: 5) #=> "Grocery, Books, Health & Beauty"
Commerce:;department(max: 2, fixed_amount: true) #=> "Books & Tools"

Commerce::material() //=> "Plastic"

Commerce::product_name() //=> "Practical Granite Shirt"

# Produces a Float by default
# Keyword arguments: range, as_string
Commerce::price() //=> 44.6
Commerce:;price(range: 0..10.0, as_string: true) #=> "2.18"

# Generate a random promotion code.
# Keyword arguments: digits
Commerce::promotion_code() //=> "AmazingDeal829102"
Commerce:;promotion_code(digits: 2) #=> "AmazingPrice57"

# Generate a random brand
Commerce::brand() //=> "Apple"

# Generate a random vendor
Commerce::vendor() //=> "Walmart"
```
