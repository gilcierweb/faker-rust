# faker::default::stripe

test stripe transactions without hardcoding card numbers and tokens

```rust
stripe::valid_card() //=> "4242424242424242"
stripe::valid_card() //=> "4000056655665556"

stripe::valid_token() //=> "tok_visa"
stripe::valid_token() //=> "tok_mastercard_debit"

stripe::invalid_card() //=> "4000000000000002"
stripe::invalid_card() //=> "4000000000000010"

stripe::month() //=> "10"

stripe::year() //=> "2018" # this will always be a year in the future

stripe::ccv() //=> "123"
stripe::ccv() //=> "1234"
```

protip:
use some of the other handy faker classes for stripe charge amounts and email.

```rust
number::between() //=> 100

internet::email() //=> "niesha@swaniawski-lynch::test"
```
