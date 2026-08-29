use std::collections::VecDeque;

use crate::{coord::Coord, game::Directions};

#[derive(Debug, Default, PartialEq)]
pub struct Snake {
    // coord: Coord,
    body: VecDeque<Coord>,
    direc: Directions,
    speed_x: i64,
    speed_y: i64,
}

impl Snake {
    pub fn new() -> Self {
        let initial_head = Coord::new(0, 0);
        let initial_body1 = Coord::new(-1, 0);
        let initial_body2 = Coord::new(-2, 0);
        let initial_body3 = Coord::new(-3, 0);
        let initial_tail = Coord::new(-4, 0);
        Snake {
            // coord: Coord::new(0.0, 0.0),
            body: VecDeque::from([
                initial_head,
                initial_body1,
                initial_body2,
                initial_body3,
                initial_tail,
            ]),
            direc: Directions::Right,
            speed_x: 1,
            speed_y: 1,
        }
    }

    pub fn getter_head_coord(&self) -> (f64, f64) {
        self.body.front().unwrap().get_coords_to_canvas()
    }

    pub fn getter_body(&self) -> Vec<(i64, i64)> {
        let body = self.body.iter().map(|coord| Coord::get_coords(&coord));
        body.collect()
    }

    pub fn getter_body_to_canvas(&self) -> Vec<(f64, f64)> {
        let body = self
            .body
            .iter()
            .map(|coord| Coord::get_coords_to_canvas(&coord));
        body.collect()
    }

    pub fn move_snake(&mut self, grow_snake: bool) {
        let new_position = Coord::advance(
            self.body.front().unwrap(),
            self.direc,
            self.speed_x,
            self.speed_y,
        );
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
