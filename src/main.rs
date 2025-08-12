mod road;
use road::*;
use sdl2::pixels::Color;
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

    let mut intersection = Intersection::new(Road::new(width as i32, Direction::Horizontal, 35),
            Road::new(height as i32, Direction::Vertical, 35)
    );
    
    
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
                    let car = Car::new(width as i32/2 + 1, height as i32 - 33, 33, Color::RGB(0, 0, 255), 0, -2);
                    intersection.road2.lane2.cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let car = Car::new(width as i32/2 - 33 , 0, 33, Color::RGB(0, 0, 255), 0, 2);
                    intersection.road2.lane1.cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let car = Car::new(0, height as i32/2 + 1, 33, Color::RGB(0, 0, 255), 2, 0);
                    intersection.road1.lane2.cars.push(car);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let car = Car::new(width as i32 - 33, height as i32/2 - 33, 33, Color::RGB(0, 0, 255), -2, 0);
                    intersection.road1.lane1.cars.push(car);
                },
                _ => {}
            }
        }


        intersection.draw(&mut canvas);
        intersection.road2.lane2.cars.iter_mut().enumerate().for_each(|(i, car)| {
            car.draw(&mut canvas);
            car.mo_ve(intersection.road2.len, i);
        });

        intersection.road2.lane1.cars.iter_mut().enumerate().for_each(|(i, car)| {
            car.draw(&mut canvas);
            car.mo_ve(intersection.road2.len, i);
        });

        intersection.road1.lane1.cars.iter_mut().enumerate().for_each(|(i, car)| {
            car.draw(&mut canvas);
            car.mo_ve(intersection.road2.len, i);
        });

        intersection.road1.lane2.cars.iter_mut().enumerate().for_each(|(i, car)| {
            car.draw(&mut canvas);
            car.mo_ve(intersection.road2.len, i);
        });


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}