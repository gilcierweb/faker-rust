
# faker::Vehicle

Available since version 1.6.4.

```rust
Vehicle::vin() //=> "LLDWXZLG77VK2LUUF"

# Random vehicle manufacturer
Vehicle::manufacturer() //=> "Lamborghini"

Vehicle::make() //=> "Honda"

# Random vehicle model
# Keyword arguments: make_of_model
Vehicle::model() //=> "A8"
Vehicle:;model(make_of_model: 'Toyota') #=> "Prius"

# Random vehicle make and model
Vehicle::make_and_model() //=> "Dodge Charger"

# Random vehicle version
Vehicle::version() //=> "2.5 Turbo"

Vehicle::version() //=> "Altis Hybrid 1.8 16V CVT"

# Random vehicle color
Vehicle::color() //=> "Red"

# Random vehicle transmission
Vehicle::transmission() //=> "Automanual"

# Random vehicle drive type
Vehicle::drive_type() //=> "4x2/2-wheel drive"

# Random vehicle fuel type
Vehicle::fuel_type() //=> "Diesel"

# Random vehicle style
Vehicle::style() //=> "ESi"

# Random car type
Vehicle::car_type() //=> "Sedan"

# Random car options
Vehicle::car_options() //=> ["DVD System", "MP3 (Single Disc)", "Tow Package", "CD (Multi Disc)", "Cassette Player", "Bucket Seats", "Cassette Player", "Leather Interior", "AM/FM Stereo", "Third Row Seats"]

# Random standard car specs
Vehicle::standard_specs() //=> ["Full-size spare tire w/aluminum alloy wheel", "Back-up camera", "Carpeted cargo area", "Silver accent IP trim finisher -inc: silver shifter finisher", "Back-up camera", "Water-repellent windshield & front door glass", "Floor carpeting"]

# Random number of doors
Vehicle::doors() //=> 1
Vehicle::door_count() //=> 3

# Random engine size
Vehicle::engine_size() //=> 6
Vehicle::engine() //=> 4

# Random car year
# Between 1 and 15 years ago
Vehicle::year() //=> 2008

# Random mileage/kilometrage
# Keyword arguments: min, max
Vehicle::mileage() //=> 26961
Vehicle:;mileage(min: 50_000) #=> 81557
Vehicle:;mileage(min: 50_000, max: 250_000) #=> 117503
Vehicle::kilometrage() //=> 35378

# Random vehicle license plate (USA by default)
# Keyword arguments: state_abbreviation
Vehicle::license_plate() //=> "DEP-2483"
Vehicle:;license_plate(state_abbreviation: 'FL') #=> "977 UNU"

# Random vehicle license plate for Singapore (if locale is set)
Vehicle::singapore_license_plate() //=> "SLV1854M"
```
