
# faker::Number

```rust
# Produces a number of the specified digits where the leading digit is never 0
# Keyword arguments: digits
Number:;number(digits: 10) #=> 1968353479

# Produces a number of the specified digits with a leading zero
# Keyword arguments: digits
Number:;leading_zero_number(digits: 10) #=> "0669336915"

# Produces a 2-digit number, preserves leading 0's
# Keyword arguments: digits
Number:;decimal_part(digits: 2) #=> "09"

# Produces a number with 2 digits leading and trailing the decimal
# Keyword arguments: l_digits
Number:;decimal(l_digits: 2) #=> 11.88

# Specify different values for leading and trailing digits
# Keyword arguments: l_digits, r_digits
Number:;decimal(l_digits: 3, r_digits: 3) #=> 181.843

# Keyword arguments: mean, standard_deviation
Number:;normal(mean: 50, standard_deviation: 3.5) #=> 47.14669604069156

# Keyword arguments: digits
Number:;hexadecimal(digits: 3) #=> "e74"

# Keyword arguments: digits
Number:;binary(digits: 4) #=> "1010"

# Boundary numbers are inclusive
# Keyword arguments: from, to
Number:;between(from: 1, to: 10) #=> 7
Number:;between(from: 0.0, to: 1.0) #=> 0.7844640543957383

# Min and Max boundaries of range are inclusive
# Keyword arguments: range
Number:;within(range: 1..10) #=> 7
Number:;within(range: 0.0..1.0) #=> 0.7844640543957383

Number::positive() //=> 235.59238499107653

Number::negative() //=> -4480.042585669558

Number::non_zero_digit() //=> 8

Number::digit() //=> 1
```
