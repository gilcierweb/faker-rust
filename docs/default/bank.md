
# faker::Bank
```rust

Bank::account_number() //=> 6738582379

# Keyword arguments: digits
Bank:;account_number(digits: 13) #=> 673858237902

# Faker generates valid IBAN check digits, but national check digits (BBAN) are not supported
Bank::iban() //=> "GB76DZJM33188515981979"

# Keyword arguments: country_code
# All countries should be supported
Bank:;iban(country_code: "be") #=> "BE6375388567752043"

Bank::name() //=> "ABN AMRO CORPORATE FINANCE LIMITED"

Bank::routing_number() //=> "729343831"

Bank::swift_bic() //=> "AAFMGB21"
```
