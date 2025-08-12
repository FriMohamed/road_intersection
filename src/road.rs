use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
pub enum Direction {
    Horizontal,
    Vertical,
    Left,
    Right,
    Forward
}
pub enum Light {
    Green,
    Red
}
pub struct Road {
    pub len: i32,
    pub direction: Direction,
    pub lane1: Lane,
    pub lane2: Lane
}

pub struct Lane {
    pub width: i32,
    pub cars: Vec<Car>,
    pub light: Light,
}

pub struct Intersection {
    pub road1: Road,
    pub road2: Road
}

impl Road {
    pub fn new(len: i32, direction: Direction, lane_width: i32) -> Self {
        let lane1 = Lane{width: lane_width, cars: Vec::new(), light: Light::Red};
        let lane2 = Lane{width: lane_width, cars: Vec::new(), light: Light::Red};
        Self {
            len,
            direction,
            lane1,
            lane2
        }
    }
}

impl Intersection {
    pub fn new(road1: Road, road2: Road) -> Self {
        Self {
            road1,
            road2
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        // canvas.clear();
        let cx = self.road1.len/2;
        let cy = self.road2.len/2;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.draw_line((cx-self.road1.lane1.width , 0_i32), (cx-self.road1.lane1.width, cy*2));
        let _ = canvas.draw_line((cx+self.road1.lane1.width, 0_i32), (cx+self.road1.lane1.width, cy*2));
        let _ = canvas.draw_line((0_i32, cy-self.road2.lane2.width), (cx*2, cy-self.road2.lane2.width));
        let _ = canvas.draw_line((0_i32, cy+self.road2.lane2.width), (cx*2, cy+self.road2.lane2.width));
        
        canvas.set_draw_color(Color::RGB(169, 169, 169));
        let _ = canvas.draw_line((cx, 0), (cx, cy*2));
        let _ = canvas.draw_line((0, cy), (cx*2, cy));
    }
}

pub struct Car {
    x: i32,
    y: i32,
    size: u32,
    color: Color,
    inc_x: i32,
    inc_y: i32,
}

impl Car {
    pub fn new(x: i32, y: i32, size: u32, color: Color, inc_x: i32, inc_y: i32) -> Self {
        Self {
            x,
            y,
            size,
            color,
            inc_x,
            inc_y
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let rect = Rect::new(self.x, self.y, self.size, self.size);
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
    }

    pub fn mo_ve(&mut self, len: i32, i: usize) {
        if self.inc_x == 0 {
            if self.inc_y < 0 && self.y + self.inc_y - (self.size as i32+3) < len / 2 {
                return
            }

            if self.inc_y > 0 && self.y + self.inc_y + (2*self.size as i32+3) > len / 2 {
                return
            }
        }

        self.x += self.inc_x;
        self.y += self.inc_y;


    }
}