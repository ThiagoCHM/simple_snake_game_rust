use piston_window::*;
use rand::{thread_rng, Rng};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::LinkedList;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

const WINDOW_SIZE: [u32; 2] = [640, 480];
const GRID_SIZE: f64 = 20.0;
const INITIAL_SPEED: f64 = 0.1;
const JUMP_KEY: Key = Key::Space;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Snake {
    body: LinkedList<[i32; 2]>,
    direction: Direction,
    has_jumped: bool,
}

impl Snake {
    fn move_forward(&mut self) {
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

    fn grow(&mut self) {
        let tail = *self.body.back().expect("Snake has no tail");
        self.body.push_back(tail);
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.direction != new_direction
            && (self.direction == Direction::Left && new_direction != Direction::Right
                || self.direction == Direction::Right && new_direction != Direction::Left
                || self.direction == Direction::Up && new_direction != Direction::Down
                || self.direction == Direction::Down && new_direction != Direction::Up)
        {
            self.direction = new_direction;
        }
    }

    fn collides_with_self(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body.iter().skip(1).any(|&segment| segment == *head)
    }

    fn collides_with_wall(&self) -> bool {
        let head = self.body.front().unwrap();
        head[0] < 0
            || head[1] < 0
            || head[0] >= (WINDOW_SIZE[0] as i32 / GRID_SIZE as i32)
            || head[1] >= (WINDOW_SIZE[1] as i32 / GRID_SIZE as i32)
    }
}

struct Game {
    snake: Snake,
    food: [i32; 2],
    score: u32,
    speed: f64,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            snake: Snake {
                body: LinkedList::from_iter(vec![[5, 5], [5, 6], [5, 7]].into_iter()),
                direction: Direction::Right,
                has_jumped: false,
            },
            food: Game::generate_food(),
            score: 0,
            speed: INITIAL_SPEED,
            game_over: false,
        }
    }

    fn generate_food(&self) -> [i32; 2] {
        let mut rng = thread_rng();
        loop {
            let new_food = [
                rng.gen_range(0..(WINDOW_SIZE[0] as i32 / GRID_SIZE as i32)),
                rng.gen_range(0..(WINDOW_SIZE[1] as i32 / GRID_SIZE as i32)),
            ];
            if !self.snake.body.contains(&new_food) {
                return new_food;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.snake.move_forward();

        if self.snake.collides_with_wall() || self.snake.collides_with_self() {
            self.game_over = true;
            play_sound("assets/audio/game_over.wav");
            return;
        }

        if self.snake.body.front() == Some(&self.food) {
            self.snake.grow();
            self.food = self.generate_food();
            self.score += 100;
            play_sound("assets/audio/eat.wav");
            if self.score % 1000 == 0 {
                self.speed += 0.02;
            }
        }
    }

    fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        for block in &self.snake.body {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [
                    block[0] as f64 * GRID_SIZE,
                    block[1] as f64 * GRID_SIZE,
                    GRID_SIZE,
                    GRID_SIZE,
                ],
                context.transform,
                graphics,
            );
        }

        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [
                self.food[0] as f64 * GRID_SIZE,
                self.food[1] as f64 * GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            ],
            context.transform,
            graphics,
        );

        let text_color = if self.game_over {
            [1.0, 0.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        let text_position = if self.game_over {
            (10.0, 100.0)
        } else {
            (10.0, 30.0)
        };
        let game_over_text = if self.game_over {
            format!("Game Over - Score: {}", self.score)
        } else {
            format!("Score: {}", self.score)
        };

        text::Text::new_color(text_color, 32)
            .draw(
                &game_over_text,
                glyphs,
                &context.draw_state,
                context.transform.trans(text_position.0, text_position.1),
                graphics,
            )
            .unwrap();
    }
}

fn play_sound(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(file_path).expect("Failed to open audio file");
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Failed to find 'assets' directory");
    let font = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = Glyphs::new(
        &font,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Failed to load font");

    let mut last_update_time = std::time::Instant::now();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if !game.game_over {
                match key {
                    Key::Up => game.snake.change_direction(Direction::Up),
                    Key::Down => game.snake.change_direction(Direction::Down),
                    Key::Left => game.snake.change_direction(Direction::Left),
                    Key::Right => game.snake.change_direction(Direction::Right),
                    JUMP_KEY => game.snake.has_jumped = true,
                    _ => {}
                }
            }
        }

        window.draw_2d(&event, |c, g, device| {
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        if last_update_time.elapsed().as_secs_f64() > game.speed {
            game.update();
            last_update_time = std::time::Instant::now();
        }
    }
}
