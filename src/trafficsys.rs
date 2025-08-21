use crate::vehicle::{IncomingDirection, Vehicle};
use IncomingDirection::*;

#[derive(Debug)]
pub struct TraficSystem {
    pub direction: IncomingDirection,
    pub timer: u8,
    pub duration: u8
}

pub struct VehiclesCount {
    pub north: u8,
    pub south: u8,
    pub east: u8,
    pub west: u8
}

impl TraficSystem {
    pub fn new(direction: IncomingDirection) -> Self {
        Self {
            direction,
            timer: 0,
            duration: 180,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        if self.timer == self.duration {
            self.timer = 0;
            self.change_dir();
        }
    }

    fn change_dir(&mut self) {
        match self.direction {
            North => self.direction = West,
            West  => self.direction = South,
            South => self.direction = East,
            East  => self.direction = North
        }
    }

    pub

    fn count_vehicles(vehicles: &Vec<Vehicle>, width: f32, height: f32, lane_width: f32) -> VehiclesCount {
        let mut vehicle_count = VehiclesCount {north: 0, south: 0, east: 0, west: 0};
        vehicles.iter().for_each(|v| {
            match v.incoming_dir {
                North => if v.y + v.size <= (height / 2.0) - lane_width {vehicle_count.north += 1},
                South => if v.y >= (height / 2.0) + lane_width {vehicle_count.south += 1},
                East => if v.x + v.size <= (width / 2.0) - lane_width {vehicle_count.east += 1},
                West => if v.x >= (width / 2.0) + lane_width {vehicle_count.west += 1}
            }
        });

        vehicle_count
    }

    pub fn check_congestion(&mut self, vehicles: &Vec<Vehicle>, road_info: (f32, f32, f32), safty_gap: f32) -> VehiclesCount {
        let counts = TraficSystem::count_vehicles(vehicles, road_info.0, road_info.1, road_info.2);
        let veh_size = road_info.2 * 0.8;

        if (counts.north as f32 * (veh_size + safty_gap)) + veh_size >= (road_info.1 / 2.0) - road_info.2 {
            self.direction = North;
            self.timer = 0;
        } else if (counts.south as f32 * (veh_size + safty_gap)) + veh_size >= (road_info.1 / 2.0 ) - road_info.2 {
            self.direction = South;
            self.timer = 0;
        } else if (counts.east as f32 * (veh_size + safty_gap)) + veh_size >= (road_info.0 / 2.0 ) - road_info.2 {
            self.direction = East;
            self.timer = 0;
        } else if (counts.west as f32 * (veh_size + safty_gap)) + veh_size >= (road_info.0 / 2.0 ) - road_info.2 {
            self.direction = West;
            self.timer = 0;
        }

        counts
    }

}