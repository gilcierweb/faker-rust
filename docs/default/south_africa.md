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
