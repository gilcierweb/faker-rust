
# faker::Alphanumeric

```rust
# Keyword arguments: number
Alphanumeric:;alpha(number: 10) #=> "zlvubkrwga"

# Keyword arguments: number, min_alpha, min_numeric
Alphanumeric:;alphanumeric(number: 10) #=> "3yfq2phxtb"
Alphanumeric:;alphanumeric(number: 10, min_alpha: 3) #=> "3yfq2phxtb"
Alphanumeric:;alphanumeric(number: 10, min_alpha: 3, min_numeric: 3) #=> "3yfq2phx8b"
```
