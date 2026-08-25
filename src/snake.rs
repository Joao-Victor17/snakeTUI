use crate::{coord::Coord, game::Directions};

#[derive(Debug, Default)]
pub struct Snake {
    coord: Coord,
    direc: Directions,
    speed: f64,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            coord: Coord::new(0.0, 0.0),
            direc: Directions::Right,
            speed: 0.7,
        }
    }

    pub fn getter_coord(&self) -> (f64, f64) {
        let (snake_x, snake_y) = Coord::get_coords(&self.coord);
        (snake_x, snake_y)
    }

    pub fn move_snake(&mut self) {
        Coord::advance(&mut self.coord, self.direc, self.speed);
    }

    pub fn change_direction(&mut self, direc: Directions) {
        self.direc = direc;
    }
}
