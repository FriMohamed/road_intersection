use macroquad::prelude::*;
mod road;
mod vehicle;
use road::*;
use vehicle::{Vehicle, IncomingDirection::*};

#[macroquad::main("Road Intersection")]
async fn main() {
    let lane_width = 40.0;
    let speed = 2.0;
    let mut vehicles = Vec::<Vehicle>::new();

    loop {
        let width = screen_width();
        let height = screen_height();

        vehicles.retain(|vehicle| {
            !(vehicle.x < - lane_width || vehicle.x > width + lane_width || vehicle.y < -lane_width || vehicle.y > height + lane_width)
        });

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) {
            vehicles.push(Vehicle::new(width, height, lane_width, speed, South));
        }

        if is_key_pressed(KeyCode::Down) {
            vehicles.push(Vehicle::new(width, height, lane_width, speed, North));
        }

        if is_key_pressed(KeyCode::Left) {
            vehicles.push(Vehicle::new(width, height, lane_width, speed, West));
        }

        if is_key_pressed(KeyCode::Right) {
            vehicles.push(Vehicle::new(width, height, lane_width, speed, East));
        }

        if is_key_pressed(KeyCode::R) {
            vehicles.push(Vehicle::random(width, height, lane_width, speed));
        }

        vehicles.iter_mut().for_each(|vehicle| {
            vehicle.draw();
            vehicle.mo_ve(width, height, lane_width);
        });

        draw_roads(width, height, lane_width);
        next_frame().await;
    }
}

