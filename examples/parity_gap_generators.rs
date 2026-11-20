//! Parity Gap Example
//! Demonstrates the 39 gem-documented generators added after the initial audit.

use faker_rust::{
    barcode, beer, chile_rut, crypto_coin, drone, food, japanese_media::kamen_rider, job, verb, x,
};

fn main() {
    println!("=== Faker-Rust Parity Gap Examples ===\n");

    println!("LOCALE-BACKED:");
    println!("  Beer brand:        {}", beer::brand());
    println!("  Food sushi:        {}", food::sushi());
    println!("  Education level:   {}", job::education_level());
    println!("  Employment type:   {}", job::employment_type());
    println!("  Verb base:         {}", verb::base());
    println!("  Past participle:   {}", verb::past_participle());
    println!("  Simple present:    {}", verb::simple_present());
    println!();

    println!("BARCODE:");
    println!("  ISMN:              {}", barcode::ismn());
    println!("  ISSN:              {}", barcode::issn());
    println!(
        "  UPC-A composite:   {}",
        barcode::upc_a_with_composite_symbology()
    );
    println!(
        "  UPC-E composite:   {}",
        barcode::upc_e_with_composite_symbology()
    );
    println!();

    println!("DRONE:");
    println!("  Weight:            {}", drone::weight());
    println!("  Max ascent:        {}", drone::max_ascent_speed());
    println!("  Max descent:       {}", drone::max_descent_speed());
    println!("  Flight time:       {}", drone::flight_time());
    println!("  Max altitude:      {}", drone::max_altitude());
    println!("  Max distance:      {}", drone::max_flight_distance());
    println!("  Max speed:         {}", drone::max_speed());
    println!("  Wind resistance:   {}", drone::max_wind_resistance());
    println!("  Angular velocity:  {}", drone::max_angular_velocity());
    println!("  Tilt angle:        {}", drone::max_tilt_angle());
    println!("  Operating temp:    {}", drone::operating_temperature());
    println!("  Battery capacity:  {}", drone::battery_capacity());
    println!("  Battery voltage:   {}", drone::battery_voltage());
    println!("  Battery type:      {}", drone::battery_type());
    println!("  Battery weight:    {}", drone::battery_weight());
    println!("  Charging temp:     {}", drone::charging_temperature());
    println!("  Charging power:    {}", drone::max_charging_power());
    println!("  ISO:               {}", drone::iso());
    println!("  Max resolution:    {}", drone::max_resolution());
    println!("  Photo format:      {}", drone::photo_format());
    println!("  Video format:      {}", drone::video_format());
    println!("  Shutter range:     {}", drone::shutter_speed_range());
    println!("  Max shutter:       {}", drone::max_shutter_speed());
    println!("  Min shutter:       {}", drone::min_shutter_speed());
    println!();

    println!("STRUCTURED / STATEFUL:");
    println!("  Coin hash:         {}", crypto_coin::coin_hash());
    let rut = chile_rut::rut();
    println!("  Generated RUT:     {rut}");
    println!("  Last RUT:          {:?}", chile_rut::last_rut());
    println!("  Kamen Rider eras:  {:?}", kamen_rider::eras());
    println!("  X user:            {}", x::user());
}
