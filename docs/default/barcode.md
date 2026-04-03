# faker_rust::default::barcode

generates ean, upc, isbn, ismn, issn format barcode with check digit attached at last

```rust
barcode::ean                                      => "85657526"
barcode::ean(8)                                   => "30152700"
barcode::ean(13)                                  => "2115190480285"

barcode::ean_with_composite_symbology             => "41007624|jhoc6649"
barcode::ean_with_composite_symbology(8)          => "38357961|xuyj3266"
barcode::ean_with_composite_symbology(13)         => "9530722443911|ckhwqhid"

barcode::upc_a                                    => "766807541831"

barcode::upc_a_with_composite_symbology           => "790670155765|jovg6208"

barcode::upc_e                                    => "03746820"

barcode::upc_e_with_composite_symbology           => "05149247|bkzx9722"

barcode::isbn                                     => "9798363807732"

barcode::ismn                                     => "9790527672897"

barcode::issn                                     => "9775541703338"
```
