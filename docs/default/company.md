
# faker::Company

```rust
Company::name() //=> "Hirthe-Ritchie"

Company::suffix() //=> "Group"

Company::industry() //=> "Information Services"

# Generate a buzzword-laden catch phrase.
Company::catch_phrase() //=> "Business-focused coherent parallelism"

Company::buzzword() //=> "Business-focused"

# When a straight answer won't do, BS to the rescue!
Company::bs() //=> "empower one-to-one web-readiness"

# Generate US employee identification numbers
Company::ein() //=> "34-8488813"

# Generate "Data Universal Numbering System"
Company::duns_number() //=> "08-341-3736"

# Get a random company logo url in PNG format.
Company::logo() //=> "https://pigment.github.io/fake-logos/logos/medium/color/5.png"

Company::type() //=> "Privately Held"

# Get a random profession
Company::profession() //=> "firefighter"

# Get a random department
Company::department() //=> "Information Technology"

###############################################################
# Generate country specific identifiers
###############################################################

# Get a random Swedish organisation number
Company::swedish_organisation_number() //=> "7962578022"

# Get a random Czech organisation number
Company::czech_organisation_number() //=> "77778171"

# Get a random French siren number
Company::french_siren_number() //=> "819489626"

# Get a random French siret number
Company::french_siret_number() //=> "81948962600013"

# Get a random Norwegian organisation number
Company::norwegian_organisation_number() //=> "839071558"

# Get a random Australian organisation number
Company::australian_business_number() //=> "81137773602"

# Get a random Spanish organisation number
Company::spanish_organisation_number() //=> "P2344979"

# Get a random Polish taxpayer identification number
Company::polish_taxpayer_identification_number() //=> "1060000062"

# Get a random Polish register of national economy number
Company::polish_register_of_national_economy() //=> "123456785"

# Get a random South African company registration number
Company::south_african_pty_ltd_registration_number() //=> "5301/714689/07"
Company::south_african_close_corporation_registration_number() //=> "CK74/7585/23"
Company::south_african_listed_company_registration_number() //=> "7039/3135/06"
Company::south_african_trust_registration_number() //=> "IT38/6489900"

# Get a random Brazilian company number (CNPJ)
Company::brazilian_company_number() //=> "18553414000618"

# Get a random formatted Brazilian company number (CNPJ)
# Keyword arguments: formatted
Company:;brazilian_company_number(formatted: true) #=> "00.000.000/0000-00"

# Get a random USA Standard Industrial Classification code (SIC)
Company::sic_code() //=> "0851"

# Get a random Russian tax number:
Company::russian_tax_number() //=> "0965855857"
Company:;russian_tax_number(region: '77') #=> "7717152803"
Company:;russian_tax_number(type: :individual) #=> "488935903348"
Company:;russian_tax_number(region: '77', type: :individual) #=> "779124694601"

# Get a random formatted Indian tax number (GST)
Company::indian_gst_number() //=> "15VQPNZ2126J2ZU"
Company:;indian_gst_number(state_code: "22") #=> "22ZVWEY6632K0ZN"
```
