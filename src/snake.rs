use std::collections::VecDeque;

use crate::{coord::Coord, game::Directions};

#[derive(Debug, Default, PartialEq)]
pub struct Snake {
    // coord: Coord,
    body: VecDeque<Coord>,
    direc: Directions,
    speed: f64,
}

impl Snake {
    pub fn new() -> Self {
        let initial_head = Coord::new(0.0, 0.0);
        let initial_body = Coord::new(-1.0, 0.0);
        let initial_tail = Coord::new(-2.0, 0.0);
        Snake {
            // coord: Coord::new(0.0, 0.0),
            body: VecDeque::from([initial_head, initial_body, initial_tail]),
            direc: Directions::Right,
            speed: 0.7,
        }
    }

    pub fn getter_head_coord(&self) -> (f64, f64) {
        self.body.front().unwrap().get_coords()
    }

    pub fn getter_body(&self) -> Vec<(f64, f64)> {
        let body = self.body.iter().map(|coord| Coord::get_coords(&coord));
        body.collect()
    }

    pub fn move_snake(&mut self, grow_snake: bool) {
        let new_position = Coord::advance(self.body.front().unwrap(), self.direc, self.speed);
        if grow_snake {
            self.body.push_front(new_position);
        } else {
            self.body.push_front(new_position);
            self.body.pop_back();
        }
    }

    pub fn change_direction(&mut self, direc: Directions) {
        if direc == Directions::is_opposite(&self.direc) {
        } else {
            self.direc = direc;
        }
    }
}
