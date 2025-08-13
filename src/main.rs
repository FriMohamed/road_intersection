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
    let lane_width = 35;
    let car_size = lane_width - 2;
    let window = video_subsystem.window("road intersection", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars = Vec::<Car>::new();
    // let mut rng = rand::rng();
    let directions = [("left", Color::RGB(0, 0, 255)), ("right", Color::RGB(255, 255, 0)), ("forward", Color::RGB(128, 0, 128))];
    let mut trafic_sys = TraficSystem::new(Direction::Up);
    let h = height as i32;
    let w = width as i32;
    let lights = [(Rect::new((w/2 )-(car_size*2 +3), (h/2 )-(car_size*2+3), car_size as u32, car_size as u32), Direction::Down),
        (Rect::new((w/2 )+(37), (h/2 )-(car_size*2+3), car_size as u32, car_size as u32), Direction::Left),
        (Rect::new((w/2 )+(37), (h/2 )+(37), car_size as u32, car_size as u32), Direction::Up),
        (Rect::new((w/2 )-(car_size*2 +3), (h/2 )+(36), car_size as u32, car_size as u32), Direction::Right)
    ];
    
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            cars.retain(|car| {
                !(car.x < -50 || car.x > w + 50 || car.y < -50 || car.y > h + 50)
            });

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if should_spawn(&cars, "up", height as i32 - car_size/2, car_size) {
                        let i = rand::random_range(0..3);
                        let car = Car::new(width as i32/2 + 1, height as i32 - car_size/2, car_size as u32, "up".to_string(), directions[i].0.to_string(),  directions[i].1);
                        cars.push(car);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if should_spawn(&cars, "down", - car_size/2, car_size) {
                        let i = rand::random_range(0..3);
                        let car = Car::new(width as i32/2 - car_size , -car_size/2, car_size as u32, "down".to_string(), directions[i].0.to_string(),  directions[i].1);
                        cars.push(car);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if should_spawn(&cars, "right", - car_size/2, car_size) {
                        let i = rand::random_range(0..3);
                        let car = Car::new(-car_size/2, height as i32/2 + 1, car_size as u32, "right".to_string(), directions[i].0.to_string(),  directions[i].1);
                        // println!("{}", car.dir_atfer_int);
                        cars.push(car);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if should_spawn(&cars, "left", width as i32 - (car_size/2), car_size) {
                        let i = rand::random_range(0..3);
                        let car = Car::new(width as i32 - (car_size/2), height as i32/2 - car_size, car_size as u32, "left".to_string(), directions[i].0.to_string(),  directions[i].1);
                        // println!("{}", car.dir_atfer_int);
                        cars.push(car);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if should_spawn(&cars, "left", width as i32 - (car_size/2), car_size) {
                        let j = rand::random_range(0..4);
                        let i = rand::random_range(0..3);
                        let car = match j {
                            0 => Car::new(width as i32/2 + 1, height as i32 - car_size/2, car_size as u32, "up".to_string(), directions[i].0.to_string(),  directions[i].1),
                            1 => Car::new(width as i32/2 - car_size , -car_size/2, car_size as u32, "down".to_string(), directions[i].0.to_string(),  directions[i].1),
                            2 => Car::new(-car_size/2, height as i32/2 + 1, car_size as u32, "right".to_string(), directions[i].0.to_string(),  directions[i].1),
                            _ => Car::new(width as i32 - (car_size/2), height as i32/2 - car_size, car_size as u32, "left".to_string(), directions[i].0.to_string(),  directions[i].1)
                        };

                        cars.push(car);
                    }
                },
                _ => {}
            }
        }

        draw_roads(&mut canvas, width as i32, height as i32, lane_width);
        draw_light(&mut canvas, &lights, &trafic_sys);
        let cars_clone =  cars.clone();
        trafic_sys.update();
        cars.iter_mut().for_each(|car| {
            car.draw(&mut canvas);
            car.mo_ve(width as i32, height as i32, lane_width, &trafic_sys.direction, cars_clone.clone());
        });


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn should_spawn(cars: &Vec<Car>, dir: &str, limit: i32, car_size: i32) -> bool {
    for c in cars {
        if (dir == "up" && c.direction == "up" && c.y + car_size + 3 >= limit) || (dir == "down" && c.direction == "down" && limit + car_size + 3 >= c.y) ||
            (dir == "right" && c.direction == "right" && limit + car_size + 3 >= c.x) || (dir == "left" && c.direction == "left" && c.x + car_size + 3 >= limit)
        {
            return false
        }
    }

    true
}

fn draw_roads(canvas:  &mut WindowCanvas, width: i32, height: i32, lane_width: i32) {
    let cx = width / 2;
    let cy = height / 2;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_line((cx - lane_width , 0_i32), (cx - lane_width, height));
    let _ = canvas.draw_line((cx + lane_width, 0_i32), (cx + lane_width, height));
    let _ = canvas.draw_line((0_i32, cy - lane_width), (width, cy - lane_width));
    let _ = canvas.draw_line((0_i32, cy + lane_width), (width, cy + lane_width));
        
    canvas.set_draw_color(Color::RGB(169, 169, 169));
    let _ = canvas.draw_line((0, cy), (width, cy));
    let _ = canvas.draw_line((cx, 0), (cx, height));
}

#[derive(Debug, Clone)]
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

    fn mo_ve(&mut self, w: i32, h: i32, lane_width: i32, dir: &Direction, cars: Vec<Car>) {
        if self.direction == "up" {
            for i in 0..cars.len() {
                if cars[i].direction == "up" && cars[i].y != self.y && cars[i].y < self.y  {
                    if self.y - self.speed <= cars[i].y + self.size as i32 + 3 {
                        return
                    }
                }
            }

            if !self.is_changed && self.dir_atfer_int == "right" && self.y - self.speed < h / 2 {
                self. direction = "right".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.y - self.speed < (h / 2) - lane_width {
                self. direction = "left".to_string();
                self.is_changed = true;
            } else if !((self.y - self.speed - (h/2 + lane_width)).abs()  <= 1  && *dir != Direction::Up) {
                self.y -= self.speed;
            }
        }

        if self.direction == "down" {
            for i in 0..cars.len() {
                if cars[i].direction == "down" && cars[i].y != self.y && cars[i].y > self.y  {
                    if self.y + self.speed + self.size as i32 + 3 >= cars[i].y {
                        return
                    }
                }
            }

            if !self.is_changed && self.dir_atfer_int == "right" && self.y + self.speed > h / 2 - lane_width {
                self. direction = "left".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.y + self.speed > (h / 2) {
                self. direction = "right".to_string();
                self.is_changed = true;
            } else if !(((h/2 - lane_width) - (self.y + self.size as i32 + self.speed)).abs() <= 1  && *dir != Direction::Down){
                self.y += self.speed;
            }
        }

        if self.direction == "right" {
            for i in 0..cars.len() {
                if cars[i].direction == "right" && cars[i].x != self.x && cars[i].x > self.x  {
                    if self.x + self.speed + self.size as i32 + 3 >= cars[i].x {
                        return
                    }
                }
            }

            if !self.is_changed && self.dir_atfer_int == "left" && self.x + self.speed > w / 2 {
                self. direction = "up".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "right" && self.x + self.speed > (w / 2) - lane_width {
                self. direction = "down".to_string();
                self.is_changed = true;
            } else if  !(((w/2 - lane_width) - (self.x + self.size as i32 + self.speed)).abs() <= 1  && *dir != Direction::Right) {
                self.x += self.speed;
            }
        }

         if self.direction == "left" {
            for i in 0..cars.len() {
                if cars[i].direction == "left" && cars[i].x != self.x && cars[i].x < self.x  {
                    if self.x - self.speed - self.size as i32 - 3 <= cars[i].x {
                        return
                    }
                }
            }

            if !self.is_changed && self.dir_atfer_int == "right" && self.x - self.speed < w / 2 {
                self. direction = "up".to_string();
                self.is_changed = true;
            } else if !self.is_changed && self.dir_atfer_int == "left" && self.x - self.speed < (w / 2) - lane_width {
                self. direction = "down".to_string();
                self.is_changed = true;
            } else if !(((w/2 + lane_width) - (self.x - self.speed)).abs() <= 1  && *dir != Direction::Left) {
                self.x -= self.speed;
            }
        }

    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

struct TraficSystem {
    direction: Direction,
    timer: u8,
    duration: u8
}

impl TraficSystem {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
            timer: 0,
            duration: 180,
        }
    }

    fn update(&mut self) {
        self.timer += 1;
        if self.timer == self.duration {
            self.timer = 0;
            self.change_dir();
        }
    }

    fn change_dir(&mut self) {
        match self.direction {
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down
        }
    }

}

fn draw_light(canvas: &mut WindowCanvas, lights: &[(Rect, Direction)], trafic_sys: &TraficSystem) {
    for light in lights {
        if trafic_sys.direction == light.1 {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas.fill_rect(light.0).unwrap();
    }
    
}
