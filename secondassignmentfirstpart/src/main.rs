// Assignment 1: Temperature Converter

const FREEZING_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F) * (5.0 / 9.0) // this is returned
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_F // this is returned
}

fn main() {
    let mut temperature_f: f64 = 32.0;

    let mut temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{}°F = {:.2}°C", temperature_f, temperature_c);

    for a in 1..5 {
        temperature_f = temperature_f + a as f64;
        temperature_c = fahrenheit_to_celsius(temperature_f);
        println!("{}°F = {:.2}°C", temperature_f, temperature_c);
    }
}

