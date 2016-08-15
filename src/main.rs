extern crate sdl2;
extern crate rand;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::Color;


#[derive(PartialEq)]
enum Direction { Up, Down, Left, Right }


struct Game {
    snake: Snake,
    apple: Point,
    score: u32,
}

impl Game {
    fn new() -> Game {// should take width and height (32, 24)
        Game {
            snake: Snake::new(3, 3),
            apple: random_point(),
            score: 0,
        }
    }
}


struct Snake {
    body: Vec<Point>,
    direction: Direction,
    last_direction: Direction,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        let mut body = Vec::with_capacity(15);
        body.push(Point::new(x, y));
        Snake {
            body: body,
            direction: Direction::Right,
            last_direction: Direction::Up,
        }
    }

    fn update(&mut self, grew: bool) {
        let next = self.body[self.body.len() - 1] + match self.direction {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        };

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

        head.x() < 0 || head.y() < 0 || head.x() > 31 || head.y() > 23
    }
}

fn random_point() -> Point {
    let (x, y) = rand::random::<(i32, i32)>();
    Point::new(x.abs() % 31, y.abs() % 23)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => println!("pause"),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if game.snake.direction != Direction::Down {
                        game.snake.direction = Direction::Up;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if game.snake.direction != Direction::Up {
                        game.snake.direction = Direction::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if game.snake.direction != Direction::Right {
                        game.snake.direction = Direction::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if game.snake.direction != Direction::Left {
                        game.snake.direction = Direction::Right;
                    }
                },
                _ => {}
            }
        }


        if game.snake.colliding() {
            println!("you lose");
            break 'running;
        }

        if game.snake.on_apple(&game.apple) {
            game.score += game.snake.body.len() as u32;
            println!("score: {}", game.score);
            game.apple = random_point();
            game.snake.update(true);
        } else {
            game.snake.update(false);
        }



        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(128, 0, 0));
        for point in &game.snake.body {
            renderer.fill_rect(Rect::new(point.x() * 20, point.y() * 20, 19, 19)).unwrap();
        }

        renderer.set_draw_color(Color::RGB(0, 128, 0));
        renderer.fill_rect(Rect::new(game.apple.x() * 20, game.apple.y() * 20, 19, 19)).unwrap();

        renderer.present();


        std::thread::sleep(Duration::from_millis(80));
    }
}
