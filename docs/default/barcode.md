
# faker::Barcode

Generates EAN, UPC, ISBN, ISMN, ISSN format barcode with check digit attached at last

```rust
# EAN barcodes
Barcode.ean                                      => "85657526"
Barcode:;ean(8)                                   => "30152700"
Barcode:;ean(13)                                  => "2115190480285"

# EAN barcodes with composite string attached in code
Barcode.ean_with_composite_symbology             => "41007624|JHOC6649"
Barcode:;ean_with_composite_symbology(8)          => "38357961|XUYJ3266"
Barcode:;ean_with_composite_symbology(13)         => "9530722443911|CKHWQHID"

# UPC_A barcodes
Barcode.upc_a                                    => "766807541831"

# UPC_A barcode with composite symbology attached
Barcode.upc_a_with_composite_symbology           => "790670155765|JOVG6208"

# UPC_E barcode numbers
Barcode.upc_e                                    => "03746820"

# UPC_E barcode with composite symbology attached
Barcode.upc_e_with_composite_symbology           => "05149247|BKZX9722"

# ISBN barcode numbers
Barcode.isbn                                     => "9798363807732"

# ISMN barcode numbers
Barcode.ismn                                     => "9790527672897"

# ISSN barcode numbers
Barcode.issn                                     => "9775541703338"
```
