extern crate sdl2;
extern crate rand;

use std::time::Duration;
use std::collections::VecDeque;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::Color;


#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    Lost,
}


#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_point(&self) -> Point {
        match *self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}


struct Game {
    state: GameState,
    snake: Snake,
    apple: Point,
    score: u32,
}

impl Game {
    fn new() -> Game {
        // should take width and height (32, 24)
        Game {
            state: GameState::Running,
            snake: Snake::new(3, 3),
            apple: random_point(),
            score: 0,
        }
    }

    fn handle_key_press(&mut self, key: Keycode) {
        use sdl2::keyboard::Keycode::*;
        match key {
            Space => self.state = GameState::Paused,
            Up => self.snake.change_direction(Direction::Up),
            Down => self.snake.change_direction(Direction::Down),
            Left => self.snake.change_direction(Direction::Left),
            Right => self.snake.change_direction(Direction::Right),
            _ => {}
        }
    }

    fn update(&mut self) {
        if self.snake.colliding() {
            self.state = GameState::Lost;
        }

        if self.snake.on_apple(&self.apple) {
            self.score += self.snake.body.len() as u32;
            self.apple = random_point();
            self.snake.update(true);
        } else {
            self.snake.update(false);
        }
    }
}


struct Snake {
    body: VecDeque<Point>,
    moved: Direction,
    moving: Direction,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        let mut snake = Snake {
            body: VecDeque::with_capacity(15),
            moved: Direction::Right,
            moving: Direction::Right,
        };
        snake.body.push_front(Point::new(x, y));
        snake
    }

    fn change_direction(&mut self, direction: Direction) {
        if self.moved != direction.opposite() {
            self.moving = direction;
        }
    }

    fn update(&mut self, grew: bool) {
        let next = self.body[0] + self.moving.to_point();

        if !grew {
            self.body.pop_back();
        }

        self.body.push_front(next);

        self.moved = self.moving;
    }

    fn on_apple(&self, apple: &Point) -> bool {
        self.body[0] == *apple
    }

    fn colliding(&self) -> bool {
        let mut body_iter = self.body.iter();
        let head: &Point = body_iter.next().unwrap();

        if body_iter.any(|piece| piece == head) {
            return true;
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
    let window = video_subsystem
        .window("Snake", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => game.handle_key_press(key),
                _ => {}
            }
        }

        game.update();

        if game.state == GameState::Lost {
            break 'running;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(128, 0, 0));
        for point in &game.snake.body {
            renderer
                .fill_rect(Rect::new(point.x() * 20, point.y() * 20, 19, 19))
                .unwrap();
        }

        renderer.set_draw_color(Color::RGB(0, 128, 0));
        renderer
            .fill_rect(Rect::new(game.apple.x() * 20, game.apple.y() * 20, 19, 19))
            .unwrap();

        renderer.present();

        std::thread::sleep(Duration::from_millis(80));
    }
}
