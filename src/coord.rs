// NOTE: Define o struct Coordenadas e métodos úteis para gerenciar a localização dentro do terminal
// dos objetos

use crate::game::Directions;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coord {
    x: f64,
    y: f64,
}

// TODO: Ver o que exatamente faz essa trait
impl AddAssign<i32> for Coord {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs as f64;
        self.y += rhs as f64;
    }
}

impl Coord {
    // NOTE: Método construtor de uma coordenada
    pub fn new(x: f64, y: f64) -> Self {
        Coord { x, y }
    }

    // NOTE: Métodos para movimentar uma coordenada
    pub fn advance(&self, direc: Directions, speed: f64) -> Coord {
        match direc {
            Directions::Up => Coord {
                x: self.x,
                y: self.y + speed / 2.0,
            },
            Directions::Right => Coord {
                x: self.x + speed,
                y: self.y,
            },
            Directions::Down => Coord {
                x: self.x,
                y: self.y - speed / 2.0,
            },
            Directions::Left => Coord {
                x: self.x - speed,
                y: self.y,
            },
        }
    }

    pub fn get_coords(&self) -> (f64, f64) {
        let (x, y) = (self.x, self.y);
        (x, y)
    }
}
