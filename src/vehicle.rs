use macroquad::{color::Color, shapes::{draw_rectangle}};

#[derive(Debug, Clone, Copy)]
pub enum IncomingDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum TurnDirection {
    Left,
    Forward,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
    pub color: Color,
    pub is_dir_changed: bool,
    pub turn_dir: TurnDirection,
    pub incoming_dir: IncomingDirection
}

use IncomingDirection::*;
use TurnDirection::*;

use crate::vehicle;
impl Vehicle {
    pub fn new(width: f32, height: f32, lane_width: f32, speed: f32, incoming_dir: IncomingDirection) -> Self {
        let vehicle_size = 0.8 * lane_width;
        let gap_to_center_vehicle = 0.1 * lane_width;
        let turn_dir_and_color = Self::turn_dir_and_color();
        let (x, y) = match incoming_dir {
            North => ((width / 2.0 - lane_width) + gap_to_center_vehicle, -(vehicle_size * 0.5)),
            East => (-(vehicle_size * 0.5), (height / 2.0) + gap_to_center_vehicle),
            South => ((width / 2.0) + gap_to_center_vehicle, height - (vehicle_size * 0.5)),
            West => (width - (vehicle_size * 0.5), (height / 2.0) - lane_width + gap_to_center_vehicle)
        };

        Self {
            x,
            y,
            size: vehicle_size,
            speed: speed,
            is_dir_changed: false,
            color: turn_dir_and_color.0,
            turn_dir: turn_dir_and_color.1,
            incoming_dir
        }

    }

    pub fn random(width: f32, height: f32, lane_width: f32, speed: f32) -> Self {
        let r = rand::random_range(0..4);
        match r {
            0 => Self::new(width, height, lane_width, speed, East),
            1 => Self::new(width, height, lane_width, speed, West),
            2 => Self::new(width, height, lane_width, speed, North),
            _ => Self::new(width, height, lane_width, speed, South)
        }
    }
    
    fn turn_dir_and_color() -> (Color, TurnDirection) {
        let r = rand::random_range(0..3);
        match r {
            0 => (Color{r: 1.0, g: 0.6, b: 0.0, a: 1.0}, TurnDirection::Left),
            1 => (Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0}, TurnDirection::Right),
            _ => (Color{r: 0.5, g: 0.5, b: 0.5, a: 1.0}, TurnDirection::Forward)
        }

    }

    fn is_colliding(&self, b: &Self) bool {
        match self.incoming_dir {
            South => self.x < b.x + b.size && self.x + self.size > b.x && self.y - self.speed < b.y + b.size && self.y + self.size > b.y,
            North => self.x < b.x + b.size && self.x + self.size > b.x && self.y + self.speed < b.y + b.size && self.y + self.size > b.y,
            East => self.y < b.y + b.size && self.y + self.size > b.y && self.x + self.size + self.speed > b.x && self.x < b.x + b.size,
            West => self.y < b.y + b.size && self.y + self.size > b.y && self.x + self.size - self.speed > b.x && self.x < b.x + b.size
        }
    }

    pub fn mo_ve(&mut self, width: f32, height: f32, lane_width: f32, vehicles: &Vec<Vehicle>) {
        let red_light = false;
        if (self.is_on_intersec_line(width, height, lane_width) && red_light)
            || vehicles.iter().any(|vehicle| self.is_colliding(vehicle))
        {
            return
        }

        

        if !self.is_dir_changed && self.is_on_turn_pos(width, height, lane_width) {
            self.turn();
        }
        
        if self.can_move {
            match self.incoming_dir {
                East => self.x += self.speed,
                West => self.x -= self.speed,
                North => self.y += self.speed,
                South => self.y -= self.speed
            }
        }
    }

    fn is_on_intersec_line(&self, width: f32, height: f32, lane_width: f32) -> bool {
        match self.incoming_dir {
            North => self.y + self.size + self.speed >= (height / 2.0) - lane_width,
            South => self.y - self.speed <= (height / 2.0) + lane_width,
            East => self.x + self.size + self.speed >= (width / 2.0) - lane_width,
            West => self.x - self.speed <= (width / 2.0) + lane_width
        }
    }

    fn is_on_turn_pos(&self, width: f32, height: f32, lane_width: f32) -> bool{
        let center_gap = 0.1 * lane_width;
        match self.incoming_dir {
            North => match self.turn_dir {
                Right => self.y + self.size + self.speed > (height / 2.0) - center_gap,
                Left => self.y + self.size + self.speed > (height / 2.0) + lane_width - center_gap,
                Forward => false 
            },
            South => match self.turn_dir {
                Right => self.y - self.speed < (height / 2.0) + center_gap,
                Left => self.y - self.speed < (height / 2.0) - lane_width + center_gap,
                Forward => false 
            },
            East => match self.turn_dir {
                Right => self.x + self.size + self.speed > (width / 2.0) - center_gap,
                Left => self.x + self.size + self.speed > (width / 2.0) + lane_width - center_gap,
                Forward => false 
            },
            West => match self.turn_dir {
                Right => self.x - self.speed < (width / 2.0) + center_gap,
                Left => self.x - self.speed < (width / 2.0) - lane_width + center_gap,
                Forward => false 
            }

        }
    }

    fn turn(&mut self) {
        match self.incoming_dir {
            North => match self.turn_dir {
                Left => self.incoming_dir = East,
                Right => self.incoming_dir = West,
                _ => ()
            },
            South => match self.turn_dir {
                Left => self.incoming_dir = West,
                Right => self.incoming_dir = East,
                _ => ()
            },
            West => match self.turn_dir {
                Left => self.incoming_dir = North,
                Right => self.incoming_dir = South,
                _ => ()
            },
            East => match self.turn_dir {
                Left => self.incoming_dir = South,
                Right => self.incoming_dir = North,
                _ => ()
            }
        }
        self.is_dir_changed = true;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
    }
}