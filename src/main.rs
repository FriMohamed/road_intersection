use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
// use rand::Rng;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width= 1000;
    let height = 800;
    let window = video_subsystem.window("road intersection", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars = Vec::new();
    // let mut rng = rand::rng();
    let directions = ["left", "right", "forward"];
    
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let i = rand::random_range(0..2);
                    let car = Car::new(width as i32/2 + 1, height as i32 - 33, 33, "up".to_string(), directions[i].to_string(), Color::RGB(0, 0, 255));
                    println!("{}", car.dir_atfer_int);
                    cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let i = rand::random_range(0..2);
                    let car = Car::new(width as i32/2 - 33 , 0, 33, "down".to_string(), directions[i].to_string(), Color::RGB(0, 0, 255));
                    println!("{}", car.dir_atfer_int);
                    cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let i = rand::random_range(0..2);
                    let car = Car::new(0, height as i32/2 + 1, 33, "right".to_string(), directions[i].to_string(), Color::RGB(0, 0, 255));
                    println!("{}", car.dir_atfer_int);
                    cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let i = rand::random_range(0..2);
                    let car = Car::new(width as i32 - 33, height as i32/2 - 33, 33, "left".to_string(), directions[i].to_string(), Color::RGB(0, 0, 255));
                    println!("{}", car.dir_atfer_int);
                    cars.push(car);
                },
                _ => {}
            }
        }

        draw_roads(&mut canvas, width as i32, height as i32, 35);
        cars.iter_mut().for_each(|car| {
            car.draw(&mut canvas);
            car.mo_ve(width as i32, height as i32);
        });


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_roads(canvas:  &mut WindowCanvas, width: i32, height: i32, lane_width: i32) {
    let cx = width / 2;
    let cy = height / 2;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_line((cx - lane_width , 0_i32), (cx - lane_width, cy*2));
    let _ = canvas.draw_line((cx + lane_width, 0_i32), (cx + lane_width, cy * 2));
    let _ = canvas.draw_line((0_i32, cy - lane_width), (cx * 2, cy - lane_width));
    let _ = canvas.draw_line((0_i32, cy + lane_width), (cx * 2, cy + lane_width));
        
    canvas.set_draw_color(Color::RGB(169, 169, 169));
    let _ = canvas.draw_line((0, cy), (cx * 2, cy));
    let _ = canvas.draw_line((cx, 0), (cx, cy * 2));
}

struct Car {
    x: i32,
    y: i32,
    size: u32, 
    direction: String,
    dir_atfer_int: String,
    speed: i32,
    color: Color,
    is_changed: bool
}

impl Car {
    fn new(x: i32, y: i32, size: u32, direction: String, dir_atfer_int: String, color: Color) -> Self {
        Self {
            x,
            y,
            size, 
            direction,
            dir_atfer_int,
            color,
            speed: 2,
            is_changed: false
        }
    }

    fn draw(&self, canvas:  &mut WindowCanvas) {
        let rect = Rect::new(self.x, self.y, self.size, self.size);
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
    }

    fn mo_ve(&mut self, w: i32, h: i32) {
        if self.direction == "up" {
            if !self.is_changed && self.dir_atfer_int == "right" && self.y - self.speed < h / 2 {
                self. direction = "right".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.y - self.speed < (h / 2) - 35 {
                self. direction = "left".to_string();
                self.is_changed = true;
            } else {
                self.y -= self.speed;
            }
        }

        if self.direction == "down" {
            if !self.is_changed && self.dir_atfer_int == "right" && self.y + self.speed > h / 2 - 35 {
                self. direction = "left".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.y + self.speed > (h / 2) {
                self. direction = "right".to_string();
                self.is_changed = true;
            } else {
                self.y += self.speed;
            }
        }

        if self.direction == "right" {
            if !self.is_changed && self.dir_atfer_int == "left" && self.x + self.speed > w / 2 {
                self. direction = "up".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "right" && self.x + self.speed > (w / 2) - 35 {
                self. direction = "down".to_string();
                self.is_changed = true;
            } else {
                self.x += self.speed;
            }
        }

         if self.direction == "left" {
            if !self.is_changed && self.dir_atfer_int == "right" && self.x - self.speed < w / 2 {
                self. direction = "up".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.x - self.speed < (w / 2) - 35 {
                self. direction = "down".to_string();
                self.is_changed = true;
            } else {
                self.x -= self.speed;
            }
        }

    }
}
