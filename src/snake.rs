use piston_window::*;
use std::collections::LinkedList;

const GRID_SIZE: f64 = 20.0;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: LinkedList<[i32; 2]>,
    direction: Direction,
    has_jumped: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: LinkedList::from_iter(vec![[5, 5], [5, 6], [5, 7]].into_iter()),
            direction: Direction::Right,
            has_jumped: false,
        }
    }

    pub fn move_forward(&mut self) {
        let head = *self.body.front().expect("Snake has no body");
        let new_head = match self.direction {
            Direction::Up => [head[0], head[1] - 1],
            Direction::Down => [head[0], head[1] + 1],
            Direction::Left => [head[0] - 1, head[1]],
            Direction::Right => [head[0] + 1, head[1]],
        };

        if !self.has_jumped {
            self.body.pop_back();
        }
        self.body.push_front(new_head);
        self.has_jumped = false;
    }

    pub fn grow(&mut self) {
        let tail = *self.body.back().expect("Snake has no tail");
        self.body.push_back(tail);
    }

    pub fn change_direction(&mut self, key: Key) {
        let new_direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(direction) = new_direction {
            if direction != self.direction &&
               (self.direction == Direction::Left && direction != Direction::Right ||
                self.direction == Direction::Right && direction != Direction::Left ||
                self.direction == Direction::Up && direction != Direction::Down ||
                self.direction == Direction::Down && direction != Direction::Up) {
                self.direction = direction;
            }
        }
    }

    pub fn collides_with_self(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body.iter().skip(1).any(|&segment| segment == *head)
    }

    pub fn collides_with_wall(&self) -> bool {
        let head = self.body.front().unwrap();
        head[0] < 0 || head[1] < 0 || head[0] >= (super::WINDOW_SIZE[0] as i32 / GRID_SIZE as i32) || head[1] >= (super::WINDOW_SIZE[1] as i32 / GRID_SIZE as i32)
    }

    pub fn eats_food(&self, food: [i32; 2]) -> bool {
        self.body.front() == Some(&food)
    }

    pub fn occupies(&self, pos: [i32; 2]) -> bool {
        self.body.contains(&pos)
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        for block in &self.body {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [block[0] as f64 * GRID_SIZE, block[1] as f64 * GRID_SIZE, GRID_SIZE, GRID_SIZE],
                context.transform,
                graphics,
            );
        }
    }
}