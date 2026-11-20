# faker_rust::drone::drone

Generate drone data.

```rust
use faker_rust::drone;

// Aircraft Properties
drone::name()                   //=> "DJI Mavic Air 2"
drone::weight()                 //=> "570 g"
drone::max_ascent_speed()       //=> "4 m/s"
drone::max_descent_speed()      //=> "3 m/s"
drone::flight_time()            //=> "34 min"
drone::max_altitude()           //=> "5000 m"
drone::max_flight_distance()    //=> "18500 m"
drone::max_speed()              //=> "19 m/s"
drone::max_wind_resistance()    //=> "10.5 m/s"
drone::max_angular_velocity()   //=> "250º/s"
drone::max_tilt_angle()         //=> "35º"
drone::operating_temprature()   //=> "14º-104ºF"

// Battery Properties
drone::battery_capacity()       //=> "3500 mAh"
drone::battery_voltage()        //=> "13.2V"
drone::battery_type()           //=> "LiPo 4S"
drone::battery_weight()         //=> "198 g"
drone::charging_temprature()    //=> "41º-104ºF"
drone::max_charging_power()     //=> "38W"

// Camera Properties
drone::iso()                    //=> "100-3200"
drone::max_resolution()         //=> "48MP"
drone::photo_format()           //=> "JPEG"
drone::video_format()           //=> "MP4"
drone::shutter_speed_range()    //=> "8-1/8000s"
drone::max_shutter_speed()      //=> "60"
drone::min_shutter_speed()      //=> "1/8000"
```

## Verified Rust API examples

Real outputs observed from the crate:

```rust
drone::weight() //=> "129 g"
drone::max_ascent_speed() //=> "3 m/s"
drone::max_descent_speed() //=> "3 m/s"
drone::flight_time() //=> "11 min"
drone::max_altitude() //=> "1565 m"
drone::max_flight_distance() //=> "7377 m"
drone::max_speed() //=> "66 m/s"
drone::max_wind_resistance() //=> "53.1 m/s"
drone::max_angular_velocity() //=> "99°/s"
drone::max_tilt_angle() //=> "30°"
drone::operating_temperature() //=> "17°-801°F"
drone::battery_capacity() //=> "3682 mAh"
drone::battery_voltage() //=> "82.3V"
drone::battery_type() //=> "Li-Polymer"
drone::battery_weight() //=> "277 g"
drone::charging_temperature() //=> "63°-486°F"
drone::max_charging_power() //=> "14W"
drone::iso() //=> "100-6400"
drone::max_resolution() //=> "25MP"
drone::photo_format() //=> "JPEG"
drone::video_format() //=> "MOV"
drone::shutter_speed_range() //=> "2-1/60s"
drone::max_shutter_speed() //=> "4"
drone::min_shutter_speed() //=> "1/8000"
```

The gem's generated docs spell two methods `charging_temprature` and
`operating_temprature`; both typos are exported as aliases of the correctly
spelled functions.
