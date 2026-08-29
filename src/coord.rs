// NOTE: Define o struct Coordenadas e métodos úteis para gerenciar a localização dentro do terminal
// dos objetos

use crate::game::Directions;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coord {
    x: i64,
    y: i64,
}

// TODO: Ver o que exatamente faz essa trait
impl AddAssign<i32> for Coord {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs as i64;
        self.y += rhs as i64;
    }
}

impl Coord {
    // NOTE: Método construtor de uma coordenada
    pub fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    // NOTE: Métodos para movimentar uma coordenada
    pub fn advance(&self, direc: Directions, speed_x: i64, speed_y: i64) -> Coord {
        match direc {
            Directions::Up => Coord {
                x: self.x,
                y: self.y + speed_y,
            },
            Directions::Right => Coord {
                x: self.x + speed_x,
                y: self.y,
            },
            Directions::Down => Coord {
                x: self.x,
                y: self.y - speed_y,
            },
            Directions::Left => Coord {
                x: self.x - speed_x,
                y: self.y,
            },
        }
    }

    pub fn get_coords(&self) -> (i64, i64) {
        let (x, y) = (self.x, self.y);
        (x, y)
    }

    pub fn get_coords_to_canvas(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64 / 2.0)
    }
}
