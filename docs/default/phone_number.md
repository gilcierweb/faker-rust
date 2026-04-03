# faker::default::phone_number

phone numbers generate north american numbering plan formats (with +1 as the country code). for different countries formats, see [phone numbers and locales](#phone-number-and-locales).

#

generates a phone number in a random format without the country code and it can have different dividers.

```rust
phonenumber::phone_number() //=> "(504) 113-1705"
phonenumber::phone_number() //=> "662.291.7201"
phonenumber::phone_number() //=> "9415283713"
```

#

generates a random cell phone number in a random format without the country code and it can have different dividers.

```rust
phonenumber::cell_phone() //=> "(836) 115-8995"
phonenumber::cell_phone() //=> "382-597-5739"
phonenumber::cell_phone() //=> "316.828.1822"
```

#

generates a random country code number.

```rust
phonenumber::country_code() //=> "+20"
phonenumber::country_code() //=> "+39"
phonenumber::country_code() //=> "+852"
```

#

generates a random phone number with country code.

```rust
phonenumber::phone_number_with_country_code() //=> "+55 466-746-6882"
phonenumber::phone_number_with_country_code() //=> "+81 3718219558"
phonenumber::phone_number_with_country_code() //=> "+49 140 957 9846"
```

#

generates a random cell phone number with country code.

```rust
phonenumber::cell_phone_with_country_code() //=> "+852 (190) 987-9034"
phonenumber::cell_phone_with_country_code() //=> "+64 (820) 583-6474"
phonenumber::cell_phone_with_country_code() //=> "+1 591.871.7985"
```

#

generates a random phone number in e164 format, i::e., without any dividers.

```rust
phonenumber::cell_phone_in_e164 //=> "+542024834991"
phonenumber::cell_phone_in_e164 //=> "+8522846847703"
phonenumber::cell_phone_in_e164 //=> "+649477546575"
```

#

generates a random area code.

```rust
phonenumber::area_code() //=> "201"
phonenumber::area_code() //=> "613"
phonenumber::area_code() //=> "321"
```

#

generates a random exchange code.

```rust
phonenumber::exchange_code() //=> "208"
phonenumber::exchange_code() //=> "415"
phonenumber::exchange_code() //=> "652"
```

#

generates a random extension / subscriber number. can be used for both extensions and last four digits of phone number.

```rust
phonenumber::subscriber_number() //=> "3873"
phonenumber::subscriber_number() //=> "39"
phonenumber::extension() //=> "3764"
phonenumber::extension() => "37"
```


if no locale is set, faker generates north american numbering plan formats (with +1 area code). for more accurate values when generating us or ca formats, it's
recommended to set their locales accordingly:

```rust
config::locale = 'en-us'
phonenumber::country_code() //=> "+1"
phonenumber::area_code() //=> "504"
phonenumber::exchange_code() //=> "715"


config::locale = 'en-ca'
phonenumber::country_code() //=> "+1"
phonenumber::area_code() //=> "226"
phonenumber::exchange_code() //=> "956"
```

besides us and ca formats, faker also generates phone numbers according to various locales. here are some examples:

```rust
config::locale = 'da-dk'
phonenumber::cell_phone_with_country_code // "+45 20 76 45 76"

config::locale = 'de'
phonenumber::cell_phone_with_country_code // "+49 1559-7973422"

config::locale = 'pt-br'
phonenumber::cell_phone_with_country_code // "+55 (77) 96227-7968"
```
