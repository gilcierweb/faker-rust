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
