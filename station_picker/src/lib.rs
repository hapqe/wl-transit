use std::f64::consts::PI;

use stations::STATIONS;
use wasm_bindgen::prelude::*;

mod stations;

fn distance(mut lat_0: f64, mut lon_0: f64, mut lat_1: f64, mut lon_1: f64) -> f64 {
    let deg_to_rad = PI / 180.;
    lat_0 *= deg_to_rad;
    lon_0 *= deg_to_rad;
    lat_1 *= deg_to_rad;
    lon_1 *= deg_to_rad;

    6371000.
        * (lat_0.cos() * lat_1.cos() * (lon_1 - lon_0).cos() + lat_0.sin() * lat_1.sin()).acos()
}

#[wasm_bindgen]
pub fn closest_stations(latitude: f64, longitude: f64, max_distance: f64) -> String {
    let mut stations = STATIONS
        .lines()
        .skip(1)
        .map(|line| line.split(';').collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0],
                parts[1],
                distance(
                    latitude,
                    longitude,
                    parts[4].parse::<f64>().unwrap(),
                    parts[5].parse::<f64>().unwrap(),
                ),
            )
        })
        .filter(|(_, _, dist)| dist < &max_distance)
        .collect::<Vec<_>>();

    stations.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    stations
        .iter()
        .map(|(id, name, _)| format!("{}:{}\n", id, name))
        .collect::<String>()
}
