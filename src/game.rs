use std::{
    sync::{Arc, atomic::AtomicBool, mpsc},
    thread, time,
};

use ratatui::layout::Rect;

use crate::coord::Coord;

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum Directions {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

pub enum Event {
    Input(crossterm::event::KeyEvent),
    UpdateSnakeState,
}

#[derive(Debug, Clone, Copy, Default)]
struct Bounds {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    width: f64,
    height: f64,
    border_limits: Bounds,
    game_over: Arc<AtomicBool>,
}

impl Game {
    pub fn new(width: f64, height: f64, game_over: Arc<AtomicBool>) -> Self {
        Game {
            width,
            height,
            border_limits: Bounds {
                x_min: -(width / 2.0),
                y_min: -(height / 2.0),
                x_max: width / 2.0,
                y_max: height / 2.0,
            },
            game_over,
        }
    }

    pub fn is_game_over(&self) {
        self.game_over
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn out_of_bounds(&self, snake_coord: (f64, f64)) -> bool {
        let (x, y): (f64, f64) = snake_coord;

        x < self.border_limits.x_min
            || y < self.border_limits.y_min
            || x > self.border_limits.x_max
            || y > self.border_limits.y_max
    }

    pub fn get_bounds_x_to_canvas(inner_area: Rect) -> [f64; 2] {
        [
            -(inner_area.width as f64 / 2.0),
            inner_area.width as f64 / 2.0,
        ]
    }

    pub fn get_bounds_y_to_canvas(inner_area: Rect) -> [f64; 2] {
        [
            -(inner_area.height as f64 / 2.0),
            inner_area.height as f64 / 2.0,
        ]
    }
}

// thread que será responsável por monitorar quando o usuário aperta alguma tecla
pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        if let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap() {
            tx.send(Event::Input(key_event)).unwrap()
        }
    }
}

// thread responsável por atualizar constantemente o estado do jogo em um período de tempo
pub fn update_snake_state(tx: mpsc::Sender<Event>, flag: Arc<AtomicBool>) {
    let time_in_millis = time::Duration::from_millis(50);
    loop {
        if flag.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        } else {
            thread::sleep(time_in_millis);
            tx.send(Event::UpdateSnakeState).unwrap();
        }
    }
}
