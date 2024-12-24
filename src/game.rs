use piston_window::*;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

use crate::score::Score;
use crate::snake::Snake;
use crate::utils::play_sound;

pub const WINDOW_SIZE: [u32; 2] = [640, 480];
const GRID_SIZE: f64 = 20.0;
const INITIAL_SPEED: f64 = 0.1;

pub struct Game {
    snake: Snake,
    food: [i32; 2],
    score: Score,
    speed: f64,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            snake: Snake::new(),
            food: [0, 0],
            score: Score::new(),
            speed: INITIAL_SPEED,
            game_over: false,
        };
        game.food = game.generate_food();
        game
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.snake.move_forward();

        if self.snake.collides_with_wall() || self.snake.collides_with_self() {
            self.game_over = true;
            play_sound("assets/audio/game_over.wav");
        } else if self.snake.eats_food(self.food) {
            self.snake.grow();
            self.food = self.generate_food();
            self.score.increase();
            play_sound("assets/audio/eat.wav");
            if self.score.get() % 1000 == 0 {
                self.speed += 0.02;
            }
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);
        self.snake.draw(context, graphics);
        rectangle(
            [1.0, 0.0, 0.0, 1.0],
            [
                self.food[0] as f64 * GRID_SIZE,
                self.food[1] as f64 * GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            ],
            context.transform,
            graphics,
        );
        self.score.draw(context, graphics, glyphs, self.game_over);
    }

    pub fn key_press(&mut self, key: Key) {
        self.snake.change_direction(key);
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    fn generate_food(&self) -> [i32; 2] {
        let mut rng = thread_rng();
        loop {
            let new_food = [
                rng.gen_range(0..(WINDOW_SIZE[0] as i32 / GRID_SIZE as i32)),
                rng.gen_range(0..(WINDOW_SIZE[1] as i32 / GRID_SIZE as i32)),
            ];
            if !self.snake.occupies(new_food) {
                return new_food;
            }
        }
    }
}
