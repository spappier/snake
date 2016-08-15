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

struct Snake {
    direction: Direction,
    body: Vec<Point>,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        let mut body = Vec::with_capacity(15);
        body.push(Point::new(x, y));
        Snake { direction: Direction::Right, body: body }
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
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut snake = Snake::new(15, 11);
    let mut apple: Point = random_point();
    let mut score: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => println!("pause"),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if snake.direction != Direction::Down {
                        snake.direction = Direction::Up;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if snake.direction != Direction::Up {
                        snake.direction = Direction::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if snake.direction != Direction::Right {
                        snake.direction = Direction::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if snake.direction != Direction::Left {
                        snake.direction = Direction::Right;
                    }
                },
                _ => {}
            }
        }


        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(128, 0, 0));
        for point in &snake.body {
            renderer.fill_rect(Rect::new(point.x() * 20, point.y() * 20, 19, 19)).unwrap();
        }

        renderer.set_draw_color(Color::RGB(0, 128, 0));
        renderer.fill_rect(Rect::new(apple.x() * 20, apple.y() * 20, 19, 19)).unwrap();

        renderer.present();


        if snake.colliding() {
            println!("you lose");
            break 'running;
        }

        if snake.on_apple(&apple) {
            score += snake.body.len() as u32;
            //game_speed += 0.1;
            println!("score: {}", score);
            apple = random_point();
            snake.update(true);
        } else {
            snake.update(false);
        }

        std::thread::sleep(Duration::from_millis(80));
    }
}
