use macroquad::prelude::*;
mod road;
mod vehicle;
mod trafficsys;
use trafficsys::*;
use road::*;
use vehicle::{Vehicle, IncomingDirection, IncomingDirection::*};

#[macroquad::main(window_conf)]
async fn main() {
    let lane_width = 40.0;
    let speed = 2.0;
    let mut vehicles = Vec::<Vehicle>::new();
    let mut traffic_sys = TraficSystem::new(South);
    let safty_gap = 5.0;

    loop {
        draw_text(&format!("{:?}", traffic_sys.direction), 20.0, 20.0, 30.0, GREEN);
        let width = screen_width();
        let height = screen_height();

        vehicles.retain(|vehicle| {
            !(vehicle.x < - lane_width || vehicle.x > width + lane_width || vehicle.y < -lane_width || vehicle.y > height + lane_width)
        });

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) {
            let v = Vehicle::new(width, height, lane_width, speed, South);
            if can_spawn(&vehicles, South, v.y, safty_gap) {
                vehicles.push(v);
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let v = Vehicle::new(width, height, lane_width, speed, North);
            if can_spawn(&vehicles, North, v.y + v.size, safty_gap) {
                vehicles.push(v);
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let v = Vehicle::new(width, height, lane_width, speed, West);
            if can_spawn(&vehicles, West, v.x, safty_gap) {
                vehicles.push(v);
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let v = Vehicle::new(width, height, lane_width, speed, East);
            if can_spawn(&vehicles, East, v.x + v.size, safty_gap) {
                vehicles.push(v);
            }
        }

        if is_key_pressed(KeyCode::R) {
            vehicles.push(Vehicle::random(width, height, lane_width, speed));
        }

        draw_text(&format!("south veh => {}", traffic_sys.check_congestion(&vehicles, (width, height, lane_width), 5.0).west),20.0, 50.0, 30.0, WHITE);
        
        traffic_sys.update();
        draw_roads(width, height, lane_width);
        
        let vehicles_clone = vehicles.clone();
        vehicles.iter_mut().enumerate().for_each(|(i, vehicle)| {
            vehicle.draw();
            vehicle.mo_ve(width, height, lane_width, &vehicles_clone, i, traffic_sys.direction);
        });

        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_string(),
        window_width: 2000,
        window_height: 1400,
        window_resizable: false,
        ..Default::default()
    }
}


fn can_spawn(vehicles: &Vec<Vehicle>, incoming_dir: IncomingDirection, spawn_pos: f32, _safty_gap: f32) -> bool {
    for v in vehicles.iter() {
        if (incoming_dir == South && v.incoming_dir == South && v.y + v.size >= spawn_pos) ||
            (incoming_dir == North && v.incoming_dir == North && v.y <= spawn_pos) ||
            (incoming_dir == West && v.incoming_dir == West && v.x + v.size >= spawn_pos) ||
            (incoming_dir == East && v.incoming_dir == East && v.x <= spawn_pos)
        {
            return false
        }
    }

    true
}