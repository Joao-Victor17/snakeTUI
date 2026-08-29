use std::{
    io,
    sync::{Arc, atomic::AtomicBool, mpsc},
    thread,
};

pub mod coord;
pub mod game;
pub mod snake;

use crossterm::event::KeyCode;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::Marker::{self, HalfBlock},
    widgets::{
        Block, Padding, Widget,
        canvas::{Canvas, Points},
    },
};

use crate::{
    game::{Directions, Event, Food, Game, handle_input_events, update_snake_state},
    snake::Snake,
};

#[derive(Debug, Default)]
struct App {
    exit: bool,
    snake: Snake,
    game: Game,
    food: Food,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::UpdateSnakeState => {
                    self.move_or_grow_snake();
                    self.check_game_over();
                }
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            KeyCode::Up => self.change_snake_direction(Directions::Up),
            KeyCode::Right => self.change_snake_direction(Directions::Right),
            KeyCode::Down => self.change_snake_direction(Directions::Down),
            KeyCode::Left => self.change_snake_direction(Directions::Left),
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn move_or_grow_snake(&mut self) {
        let snake_coords = self.snake.getter_head_coord();
        let food_coords = self.food.get_coords_to_canvas();

        if snake_coords.0 == food_coords.0 && snake_coords.1 == food_coords.1 {
            self.snake.move_snake(true);
            self.food = Food::new(self.game.get_bounds());
        } else {
            self.snake.move_snake(false);
        }
    }

    fn change_snake_direction(&mut self, direc: Directions) {
        Snake::change_direction(&mut self.snake, direc);
    }

    fn check_game_over(&self) {
        if Game::out_of_bounds(&self.game, self.snake.getter_head_coord()) {
            self.game.is_game_over();
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let snake_pos = Snake::getter_body_to_canvas(&self.snake);
        let block = Block::bordered()
            .padding(Padding::ZERO)
            .border_style(Style::default().bg(Color::White));

        let inner_area = block.inner(area);

        Canvas::default()
            .block(block)
            .x_bounds(Game::get_bounds_x_to_canvas(inner_area))
            .y_bounds(Game::get_bounds_y_to_canvas(inner_area))
            .marker(Marker::HalfBlock)
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &snake_pos,
                    color: Color::Red,
                });
                ctx.draw(&Points {
                    coords: &[self.food.get_coords_to_canvas()],
                    color: Color::Red,
                });
            })
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let _ = color_eyre::install(); // Install panic hooks and error handler

    // Instancia o ratatui no modo raw (toda tecla e comando é redirecionado para o programa)
    let mut terminal = ratatui::init();

    let size = terminal.size();
    let height = size.as_ref().unwrap().height as f64;
    let width = size.unwrap().width as f64;

    let snake = Snake::new();

    let game_over = Arc::new(AtomicBool::new(false));

    let game = Game::new(width - 1.0, height - 2.0, Arc::clone(&game_over));

    let food = Food::new(game.get_bounds());

    let mut app = App {
        exit: false,
        snake,
        game,
        food,
    };

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    let tx_to_snake_state = event_tx.clone();

    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    thread::spawn(move || {
        update_snake_state(tx_to_snake_state, game_over);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}
