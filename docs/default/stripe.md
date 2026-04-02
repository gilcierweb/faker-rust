
# faker::Stripe

Test Stripe transactions without hardcoding card numbers and tokens

```rust
# Keyword arguments: card_type
Stripe::valid_card() //=> "4242424242424242"
Stripe:;valid_card(card_type: "visa_debit") #=> "4000056655665556"

# Keyword arguments: card_type
Stripe::valid_token() //=> "tok_visa"
Stripe:;valid_token(card_type: "mc_debit") #=> "tok_mastercard_debit"

# Keyword arguments: card_error
Stripe::invalid_card() //=> "4000000000000002"
Stripe:;invalid_card(card_error: "addressZipFail") #=> "4000000000000010"

Stripe::month() //=> "10"

Stripe::year() //=> "2018" # This will always be a year in the future

# Keyword arguments: card_type
Stripe::ccv() //=> "123"
Stripe:;ccv(card_type: "amex") #=> "1234"
```

ProTip:
Use some of the other handy Faker classes for Stripe charge amounts and email.

```rust
# Keyword arguments: from, to
Number:;between(from: 3, to: 10) #=> 100

Internet::email() //=> "niesha@swaniawski-lynch.test"
```
