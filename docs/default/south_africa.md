# faker_rust::south_africa

generate south african id numbers, phone numbers and company registration
numbers. delegates as needed to `idnumber`, `phonenumber` and
`company`

```rust
southafrica::id_number() //=> "6110311856083"
southafrica::valid_id_number() //=> "6110311856083"

southafrica::invalid_id_number() //=> "7018356904081"

southafrica::phone_number() //=> "010 788 5009"

southafrica::cell_phone() //=> "082 946 7470"

southafrica::pty_ltd_registration_number() //=> "5301/714689/07"
southafrica::close_corporation_registration_number() //=> "ck74/7585/23"
southafrica::listed_company_registration_number() //=> "7039/3135/06"
southafrica::trust_registration_number() //=> "it38/6489900"

southafrica::vat_number() //=> "za79494416181"
```

## Verified Rust API examples

Real outputs observed from the crate:

```rust
south_africa::cell_phone() //=> "558-733-4277"
south_africa::valid_id_number() //=> "4505082428180"
south_africa::invalid_id_number() //=> "5181415352080"
south_africa::vat_number() //=> "ZA1371611717"
south_africa::pty_ltd_registration_number() //=> "3541/6797/07"
south_africa::close_corporation_registration_number() //=> "CK02/842679/23"
south_africa::listed_company_registration_number() //=> "7361/3089967112/06"
south_africa::trust_registration_number() //=> "IT5855/837"
```
