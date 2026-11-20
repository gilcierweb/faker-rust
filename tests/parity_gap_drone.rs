//! Parity tests for the 24 gem-documented Drone generators that the initial
//! audit classified as internal helpers.

#[test]
fn parity_gap_drone_generators() {
    let numeric = [
        (faker_rust::drone::weight as fn() -> String, " g"),
        (faker_rust::drone::max_ascent_speed, " m/s"),
        (faker_rust::drone::max_descent_speed, " m/s"),
        (faker_rust::drone::flight_time, " min"),
        (faker_rust::drone::max_altitude, " m"),
        (faker_rust::drone::max_flight_distance, " m"),
        (faker_rust::drone::max_speed, " m/s"),
        (faker_rust::drone::max_wind_resistance, " m/s"),
        (faker_rust::drone::max_angular_velocity, "°/s"),
        (faker_rust::drone::max_tilt_angle, "°"),
        (faker_rust::drone::battery_voltage, "V"),
        (faker_rust::drone::battery_weight, " g"),
        (faker_rust::drone::max_charging_power, "W"),
        (faker_rust::drone::max_resolution, "MP"),
    ];
    for (generate, suffix) in numeric {
        let value = generate();
        assert!(!value.is_empty(), "empty drone value for suffix {suffix}");
        assert!(value.ends_with(suffix), "unexpected value: {value}");
    }

    let plain = [
        faker_rust::drone::battery_capacity as fn() -> String,
        faker_rust::drone::battery_type,
        faker_rust::drone::charging_temperature,
        faker_rust::drone::iso,
        faker_rust::drone::photo_format,
        faker_rust::drone::video_format,
        faker_rust::drone::max_shutter_speed,
        faker_rust::drone::min_shutter_speed,
        faker_rust::drone::operating_temperature,
    ];
    for generate in plain {
        assert!(!generate().is_empty());
    }

    assert!(!faker_rust::drone::weight().is_empty());
    let range = faker_rust::drone::shutter_speed_range();
    assert!(
        range.contains('-'),
        "unexpected shutter speed range: {range}"
    );
    assert!(
        range.ends_with('s'),
        "unexpected shutter speed range: {range}"
    );
}

#[test]
fn parity_gap_drone_documented_typo_aliases() {
    assert!(!faker_rust::drone::charging_temprature().is_empty());
    assert!(!faker_rust::drone::operating_temprature().is_empty());
}
