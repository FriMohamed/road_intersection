mod vehicle;
use vehicle::IncomingDirection;
use IncomingDirection::*;

#[derive(Debug)]
struct TraficSystem {
    pub direction: IncomingDirection,
    pub timer: u8,
    pub duration: u8
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
            North => self.direction = West,
            West  => self.direction = South,
            South => self.direction = East,
            East  => self.direction = North
        }
    }

}