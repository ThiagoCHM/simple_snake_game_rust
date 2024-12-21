extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    // Inicializa a janela
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake: Vec<Position> = vec![
        Position { x: 5, y: 5 }, // A cabeça da cobra
        Position { x: 4, y: 5 },
        Position { x: 3, y: 5 },
    ];

    let mut direction = Direction::Right;
    let mut food_position = generate_food(640 / 16, 480 / 16); // Gera o alimento

    let mut last_update = Instant::now();
    let update_rate = Duration::from_millis(200); // Define a velocidade da cobra

    while let Some(e) = window.next() {
        // Verifica as teclas pressionadas
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up if direction != Direction::Down => direction = Direction::Up,
                Key::Down if direction != Direction::Up => direction = Direction::Down,
                Key::Left if direction != Direction::Right => direction = Direction::Left,
                Key::Right if direction != Direction::Left => direction = Direction::Right,
                _ => {}
            }
        }

        // Atualiza a posição da cobra
        if last_update.elapsed() >= update_rate {
            update_snake(&mut snake, &direction);
            if check_collision(&snake) || check_wall_collision(&snake[0], 640 / 16, 480 / 16) {
                break; // O jogo acabou
            }

            // Verifica se a cobra comeu o alimento
            if snake[0] == food_position {
                snake.push(Position { x: -1, y: -1 }); // Adiciona um segmento à cobra
                food_position = generate_food(640 / 16, 480 / 16); // Gera um novo alimento
            }

            last_update = Instant::now();
        }

        // Desenha o fundo, a cobra e o alimento
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g); // Limpa a tela com fundo preto

            // Desenha a cobra
            for segment in &snake {
                draw_block(segment.x, segment.y, g, &c); // Passa a referência a `c`
            }

            // Desenha o alimento
            draw_block(food_position.x, food_position.y, g, &c); // Passa a referência a `c`
        });
    }
}

// Atualiza a posição da cobra
fn update_snake(snake: &mut Vec<Position>, direction: &Direction) {
    let mut new_head = snake[0]; // Copia a cabeça da cobra
    match direction {
        Direction::Up => new_head.y -= 1,
        Direction::Down => new_head.y += 1,
        Direction::Left => new_head.x -= 1,
        Direction::Right => new_head.x += 1,
    }

    snake.insert(0, new_head); // Adiciona a nova cabeça à cobra
    snake.pop(); // Remove o último segmento (a cobra não cresce ainda)
}

// Desenha um bloco (segmento da cobra ou alimento)
fn draw_block(x: i32, y: i32, g: &mut G2d, c: &Context) {
    rectangle(
        [0.0, 1.0, 0.0, 1.0], // Cor verde
        [x as f64 * 16.0, y as f64 * 16.0, 16.0, 16.0],
        c.transform,
        g,
    );
}

// Gera uma posição aleatória para o alimento
fn generate_food(grid_width: i32, grid_height: i32) -> Position {
    let mut rng = rand::thread_rng();
    Position {
        x: rng.gen_range(0..grid_width),
        y: rng.gen_range(0..grid_height),
    }
}

// Verifica se a cobra colidiu com o seu próprio corpo
fn check_collision(snake: &[Position]) -> bool {
    let head = snake[0];
    snake.iter().skip(1).any(|&segment| segment == head)
}

// Verifica se a cobra colidiu com as paredes
fn check_wall_collision(head: &Position, grid_width: i32, grid_height: i32) -> bool {
    head.x < 0 || head.y < 0 || head.x >= grid_width || head.y >= grid_height
}
