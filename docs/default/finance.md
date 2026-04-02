
# faker::Finance

```rust
# Random credit card
Finance::credit_card() //=> "3018-348979-1853"
Finance:;credit_card(:mastercard) #=> "6771-8921-2291-6236"
Finance:;credit_card(:mastercard, :visa) #=> "4448-8934-1277-7195"

# Random vat number
# Keyword arguments: country
Finance::vat_number() //=> "BR38.395.329/2471-83"
Finance:;vat_number(country: 'DE') #=> "DE593306671"
Finance:;vat_number(country: 'ZA') #=> "ZA79494416181"

# Random ticker
Finance::ticker() //=> "AMZN"
## Supported: NASDAQ, NYSE
Finance:;ticker('NASDAQ') #=> "GOOG"

# Random stock market
Finance::stock_market() //=> "NASDAQ"
```
