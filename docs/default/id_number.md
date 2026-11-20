# faker_rust::id_number

```rust
id_number::valid() //=> "552-56-3593"

id_number::invalid() //=> "311-72-0000"

id_number::spanish_citizen_number() //=> "53290236-h"

id_number::spanish_foreign_citizen_number() //=> "z-1600870-y"

id_number::south_african_id_number() //=> "8105128870184"
id_number::valid_south_african_id_number() //=> "8105128870184"

id_number::invalid_south_african_id_number() //=> "1642972065088"

id_number::brazilian_citizen_number(true) //=> "53540542221"
id_number::brazilian_citizen_number(true) //=> "535.405.422-21"

id_number::brazilian_id(true) //=> "493054029"
id_number::brazilian_id(true) //=> "49.305.402-9"

id_number::chilean_id() //=> "15620613-k"

id_number::croatian_id(false) //=> "88467617508"
id_number::croatian_id(false) //=> "hr88467617508"

id_number::danish_id_number(true) //=> "050390-9980"
id_number::danish_id_number(true) //=> "050390-9980"
id_number::danish_id_number(true) //=> "050390-9980"
id_number::danish_id_number(true) //=> "050390-9980"

id_number::french_insee_number() //=> "22510589696868"
```


besides the default id numbers, faker supports localized `.valid` and `.invalid` values:

```rust
config::locale = 'fr-fr'
id_number::valid() //=> "22510589696868"
```

locales with specific intricacies are as such:


when the locale is set to british english, unformatted [national insurance]() numbers are generated:

```rust
config::locale = 'en-gb'
id_number::valid() //=> "aj405924a"
id_number::invalid() //=> "bg316764w"


## Verified Rust API examples

Real outputs observed from the crate:

```rust
id_number::spanish_citizen_number() //=> "44422328-J"
id_number::spanish_foreign_citizen_number() //=> "Y-2453517-Y"
id_number::valid_south_african_id_number() //=> "6812097778189"
id_number::invalid_south_african_id_number() //=> "3115761768087"
id_number::brazilian_citizen_number(false) //=> "27843388001"
id_number::brazilian_citizen_number(true) //=> "132.852.626-72"
id_number::brazilian_id(false) //=> "180270941"
id_number::brazilian_id(true) //=> "20.242.252-5"
id_number::chilean_id() //=> "03107606-4"
id_number::croatian_id(false) //=> "17181325100"
id_number::croatian_id(true) //=> "HR92171948475"
id_number::danish_id_number(false) //=> "1706753940"
id_number::danish_id_number(true) //=> "140592-1334"
id_number::french_insee_number() //=> "262033696544706"
```

## Verified Rust API examples

```rust
id_number::south_african_id_number() //=> "6205103481189"
```
