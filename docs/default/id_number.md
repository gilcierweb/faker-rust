# faker_rust::id_number

```rust
idnumber::valid() //=> "552-56-3593"

idnumber::invalid() //=> "311-72-0000"

idnumber::spanish_citizen_number() //=> "53290236-h"

idnumber::spanish_foreign_citizen_number() //=> "z-1600870-y"

idnumber::south_african_id_number() //=> "8105128870184"
idnumber::valid_south_african_id_number() //=> "8105128870184"

idnumber::invalid_south_african_id_number() //=> "1642972065088"

idnumber::brazilian_citizen_number() //=> "53540542221"
idnumber::brazilian_citizen_number() //=> "535.405.422-21"

idnumber::brazilian_id() //=> "493054029"
idnumber::brazilian_id() //=> "49.305.402-9"

idnumber::chilean_id() //=> "15620613-k"

idnumber::croatian_id() //=> "88467617508"
idnumber::croatian_id() //=> "hr88467617508"

idnumber::danish_id_number() //=> "050390-9980"
idnumber::danish_id_number() //=> "050390-9980"
idnumber::danish_id_number()) //=> "050390-9980"
idnumber::danish_id_number() //=> "050390-9980"

idnumber::french_insee_number() //=> "22510589696868"
```


besides the default id numbers, faker supports localized `.valid` and `.invalid` values:

```rust
config::locale = 'fr-fr'
idnumber::valid() //=> "22510589696868"
```

locales with specific intricacies are as such:


when the locale is set to british english, unformatted [national insurance]() numbers are generated:

```rust
config::locale = 'en-gb'
idnumber::valid() //=> "aj405924a"
idnumber::invalid() //=> "bg316764w"

