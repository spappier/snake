extern crate piston_window;
extern crate rand;

use std::ops::Add;

use piston_window::*;
use rand::Rng;

#[derive(PartialEq)]
enum Direction { Up, Down, Left, Right }

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn add(&self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

struct Snake {
    direction: Direction,
    body: Vec<Point>,
}

impl Snake {
    fn new(x: f64, y: f64) -> Snake {
        let mut body = Vec::with_capacity(15);
        body.push(Point::new(x, y));
        Snake { direction: Direction::Right, body: body }
    }

    fn update(&mut self, grew: bool) {
        let next = self.body[self.body.len() - 1].add(match self.direction {
            Direction::Up => Point::new(0.0, -1.0),
            Direction::Down => Point::new(0.0, 1.0),
            Direction::Left => Point::new(-1.0, 0.0),
            Direction::Right => Point::new(1.0, 0.0),
        });

        if !grew {
            self.body.remove(0);
        }

        self.body.push(next);
    }

    fn on_apple(&self, apple: &Point) -> bool {
        &self.body[self.body.len() - 1] == apple
    }

    fn colliding(&self) -> bool {
        let head: &Point = &self.body[self.body.len() - 1].clone();

        if self.body.len() > 1 {
            for point in &self.body[1..&self.body.len() - 1] {
                if head == point {
                    return true;
                }
            }
        }

        head.x < 0.0 || head.y < 0.0 || head.x > 31.0 || head.y > 23.0
    }
}

fn random_point<R: Rng>(rng: &mut R) -> Point {
    Point { x: rng.gen_range(0, 31) as f64, y: rng.gen_range(0, 23) as f64 }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Snake", [640, 480])
        .exit_on_esc(true).into();

    let mut snake = Snake::new(15.0, 11.0);
    let mut rng = rand::thread_rng();
    let mut time_since_update: f64 = 0.0;
    let mut apple: Point = random_point(&mut rng);
    let mut score: u32 = 0;
    let mut game_speed = 10.0;

    for e in window {
        if let Some(_) = e.render_args() {
            e.draw_2d(|c, g| {
                clear([0.0; 4], g);

                for point in &snake.body {
                    rectangle([1.0, 0.0, 0.0, 1.0],
                          [point.x * 20.0, point.y * 20.0, 19.0, 19.0],
                          c.transform, g);
                }

                rectangle([1.0, 0.0, 0.0, 1.0],
                          [apple.x * 20.0, apple.y * 20.0, 19.0, 19.0],
                          c.transform, g);
            });
        }

        if let Some(args) = e.update_args() {
            time_since_update += args.dt;
            if time_since_update >= 1.0 / game_speed as f64 {
                time_since_update = 0.0;

                if snake.on_apple(&apple) {
                    score += snake.body.len() as u32;
                    game_speed += 0.1;
                    println!("score: {}", score);
                    apple = random_point(&mut rng);
                    snake.update(true);
                } else {
                    snake.update(false);
                }

                if snake.colliding() {
                    panic!("you lose");
                }

            }
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Space) => println!("pause"),
                Button::Keyboard(Key::W) | Button::Keyboard(Key::Up) => {
                    if snake.direction != Direction::Down {
                        snake.direction = Direction::Up;
                    }
                },
                Button::Keyboard(Key::S) | Button::Keyboard(Key::Down) => {
                    if snake.direction != Direction::Up {
                        snake.direction = Direction::Down;
                    }
                },
                Button::Keyboard(Key::A) | Button::Keyboard(Key::Left) => {
                    if snake.direction != Direction::Right {
                        snake.direction = Direction::Left;
                    }
                },
                Button::Keyboard(Key::D) | Button::Keyboard(Key::Right) => {
                    if snake.direction != Direction::Left {
                        snake.direction = Direction::Right;
                    }
                },
                _ => (),
            }
        }
    }
}
