# faker_rust::driving_license

Generates driving licence and driving license numbers.

```rust
use faker_rust::driving_license;

// UK licence in DVLA format: 5 surname letters, 2 year, 2 month, 2 day,
// 2 initials, then the "9AA" suffix (16 characters total).
let uk = driving_license::uk();
//=> "SMITH891231AB9AA"

// US license for a specific state, backed by locale data.
let ca = driving_license::usa("california");

// State-specific helpers.
let ny = driving_license::new_york();
let tx = driving_license::texas();
```
