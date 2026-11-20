# faker_rust::number

```rust
number::number() //=> 1968353479

number::leading_zero_number(8) //=> "0669336915"

number::decimal_part(4) //=> "09"

number::decimal(2, 2) //=> 11.88

number::decimal(2, 2) //=> 181.843

number::normal(0.0, 1.0) //=> 47.14669604069156

number::hexadecimal(8) //=> "e74"

number::binary(8) //=> "1010"

number::between(1, 10) //=> 7
number::between(1, 10) //=> 0.7844640543957383

number::within(1.0..=10.0) //=> 7
number::within(1.0..=10.0) //=> 0.7844640543957383

number::positive() //=> 235.59238499107653

number::negative() //=> -4480.042585669558

number::non_zero_digit() //=> 8

number::digit() //=> 1
```
