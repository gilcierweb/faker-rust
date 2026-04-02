
# faker::ChileRut

```rust
ChileRut::full_rut() //=> "30686957-4"

# Returns rut between 1 (default param) and 99999999
ChileRut::rut() //=> 11235813

# Returns rut between passed minimum rut and 99999999
# Keyword arguments: min_rut
ChileRut:;rut(min_rut: 20_890_156) #=> 31853211

# Every call to rut or full_rut generates a new random rut, so last_rut and dv
# allows you to get the separated parts of the full rut without losing the already generated rut
ChileRut::rut() //=> 23567131
ChileRut::last_rut() //=> 23567131
ChileRut::dv() //=> "k"

# check_digit is an alias for dv, for English speaking devs
ChileRut::rut() //=> 30528772
ChileRut::check_digit() //=> "5"

# Returns full rut
# Keyword arguments: min_rut
# Keyword arguments: fixed
ChileRut::full_rut() //=> "30686957-4"
ChileRut:;full_rut(min_rut: 20890156) #=> "20890156-4"
ChileRut:;full_rut(min_rut: 20890156, formatted: true) #=> "20.890.156-4"
ChileRut:;full_rut(min_rut: 30686957, fixed: true) #=> "30686957-4"
```
