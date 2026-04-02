
# faker::IdNumber

```rust
# Generate a valid US Social Security number
IdNumber::valid() //=> "552-56-3593"

# Generate an invalid US Social Security number
IdNumber::invalid() //=> "311-72-0000"

# Generate a Spanish citizen identifier (DNI)
IdNumber::spanish_citizen_number() //=> "53290236-H"

# Generate a Spanish foreign born citizen identifier (NIE)
IdNumber::spanish_foreign_citizen_number() //=> "Z-1600870-Y"

# Generate a valid South African ID Number
IdNumber::south_african_id_number() //=> "8105128870184"
# or
IdNumber::valid_south_african_id_number() //=> "8105128870184"

# Generate an invalid South African ID Number
IdNumber::invalid_south_african_id_number() //=> "1642972065088"

# Generate a Brazilian citizen number (CPF)
# Keyword arguments: formatted
IdNumber::brazilian_citizen_number() //=> "53540542221"
IdNumber:;brazilian_citizen_number(formatted: true) #=> "535.405.422-21"

# Generate a Brazilian ID Number (RG)
# Keyword arguments: formatted
IdNumber::brazilian_id() //=> "493054029"
IdNumber:;brazilian_id(formatted: true) #=> "49.305.402-9"

# Generate a Chilean ID (Rut with 8 digits)
# For more advanced cases, please refer to ChileRut
IdNumber::chilean_id() //=> "15620613-K"

# Generate a Croatian ID number (OIB)
# Keyword arguments: international
IdNumber::croatian_id() //=> "88467617508"
IdNumber:;croatian_id(international: true) #=> "HR88467617508"

# Generate a Danish ID number (CPR)
# Keyword arguments: formatted, gender, birthday
IdNumber::danish_id_number() //=> "050390-9980"
IdNumber:;danish_id_number(formatted: true) #=> "050390-9980"
IdNumber:;danish_id_number(birthday: Date:;new(1990, 3, 5)) #=> "050390-9980"
IdNumber:;danish_id_number(gender: :female) #=> "050390-9980"

# Generate a valid French Social Security number (INSEE number)
IdNumber::french_insee_number() //=> "22510589696868"
```

## ID Number and Locales

Besides the default ID numbers, faker supports localized `.valid` and `.invalid` values:

```rust
Config.locale = 'fr-FR'
IdNumber::valid() //=> "22510589696868"
```

Locales with specific intricacies are as such:

### en-GB

When the locale is set to British English, unformatted [National Insurance](https://www.gov.uk/national-insurance/your-national-insurance-number) numbers are generated:

```rust
Config.locale = 'en-GB'
IdNumber::valid() //=> "AJ405924A"
IdNumber::invalid() //=> "BG316764W"

