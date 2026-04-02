
# faker::PhoneNumber

Phone numbers generate North American Numbering Plan formats (with +1 as the country code). For different countries formats, see [Phone Numbers and Locales](#phone-number-and-locales).

#
# faker::PhoneNumber.phone_number

Generates a phone number in a random format without the country code and it can have different dividers.

```rust
PhoneNumber::phone_number() //=> "(504) 113-1705"
PhoneNumber::phone_number() //=> "662.291.7201"
PhoneNumber::phone_number() //=> "9415283713"
```

#
# faker::PhoneNumber.cell_phone

Generates a random cell phone number in a random format without the country code and it can have different dividers.

```rust
PhoneNumber::cell_phone() //=> "(836) 115-8995"
PhoneNumber::cell_phone() //=> "382-597-5739"
PhoneNumber::cell_phone() //=> "316.828.1822"
```

#
# faker::PhoneNumber.country_code

Generates a random country code number.

```rust
PhoneNumber::country_code() //=> "+20"
PhoneNumber::country_code() //=> "+39"
PhoneNumber::country_code() //=> "+852"
```

#
# faker::PhoneNumber.phone_number_with_country_code

Generates a random phone number with country code.

```rust
PhoneNumber::phone_number_with_country_code() //=> "+55 466-746-6882"
PhoneNumber::phone_number_with_country_code() //=> "+81 3718219558"
PhoneNumber::phone_number_with_country_code() //=> "+49 140 957 9846"
```

#
# faker::PhoneNumber.cell_phone_with_country_code

Generates a random cell phone number with country code.

```rust
PhoneNumber::cell_phone_with_country_code() //=> "+852 (190) 987-9034"
PhoneNumber::cell_phone_with_country_code() //=> "+64 (820) 583-6474"
PhoneNumber::cell_phone_with_country_code() //=> "+1 591.871.7985"
```

#
# faker::PhoneNumber.cell_phone_in_e164

Generates a random phone number in e164 format, i.e., without any dividers.

```rust
PhoneNumber.cell_phone_in_e164 #=> "+542024834991"
PhoneNumber.cell_phone_in_e164 #=> "+8522846847703"
PhoneNumber.cell_phone_in_e164 #=> "+649477546575"
```

#
# faker::PhoneNumber.area_code

Generates a random area code.

```rust
PhoneNumber::area_code() //=> "201"
PhoneNumber::area_code() //=> "613"
PhoneNumber::area_code() //=> "321"
```

#
# faker::PhoneNumber.exchange_code

Generates a random exchange code.

```rust
PhoneNumber::exchange_code() //=> "208"
PhoneNumber::exchange_code() //=> "415"
PhoneNumber::exchange_code() //=> "652"
```

#
# faker::PhoneNumber.subscriber_number (alias PhoneNumber.extension)

Generates a random extension / subscriber number. Can be used for both extensions and last four digits of phone number.

```rust
# keyword arguments: length. Defaults to 4.
PhoneNumber::subscriber_number() //=> "3873"
PhoneNumber:;subscriber_number(length: 2) #=> "39"
PhoneNumber::extension() //=> "3764"
PhoneNumber:;extension(length: 2) => "37"
```

## Phone Number and Locales

If no locale is set, Faker generates North American Numbering Plan formats (with +1 area code). For more accurate values when generating US or CA formats, it's
recommended to set their locales accordingly:

```rust
# set locale first
Config.locale = 'en-US'
PhoneNumber::country_code() //=> "+1"
PhoneNumber::area_code() //=> "504"
PhoneNumber::exchange_code() //=> "715"


# set locale first
Config.locale = 'en-CA'
PhoneNumber::country_code() //=> "+1"
PhoneNumber::area_code() //=> "226"
PhoneNumber::exchange_code() //=> "956"
```

Besides US and CA formats, Faker also generates phone numbers according to various locales. Here are some examples:

```rust
Config.locale = 'da-DK'
PhoneNumber.cell_phone_with_country_code # => "+45 20 76 45 76"

Config.locale = 'de'
PhoneNumber.cell_phone_with_country_code # => "+49 1559-7973422"

Config.locale = 'pt-BR'
PhoneNumber.cell_phone_with_country_code # => "+55 (77) 96227-7968"
```
