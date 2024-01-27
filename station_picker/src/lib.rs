use stations::STATIONS;
use wasm_bindgen::prelude::*;

mod stations;

#[wasm_bindgen]
pub fn closest_station(latitude: f64, longitude: f64) -> String {
    let closest = STATIONS
        .lines()
        .skip(1)
        .map(|line| {
            let parts: Vec<&str> = line.split(';').collect();
            let id = parts[0];
            let name = parts[1];
            let lat = parts[4].parse::<f64>().unwrap();
            let lon = parts[5].parse::<f64>().unwrap();
            let distance = (latitude - lat).powi(2) + (longitude - lon).powi(2);
            (id, name, distance)
        })
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap();

    format!("{}:{}", closest.0, closest.1)
}
