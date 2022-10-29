use crate::snake::{Direction, Snake};
use rand::prelude::*;

struct Board {
    pub width: i32,
    pub height: i32,
    pub state: Vec<Vec<i32>>,
}
impl Board {
    pub fn new(width: i32, height: i32, snake: &Snake, apple: &Apple) -> Self {
        let mut state = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(0);
            }
            state.push(row);
        }
        let head = snake.body.front().unwrap();
        state[head.y as usize][head.x as usize] = 1;
        state[apple.location.y as usize][apple.location.x as usize] = 2;
        Board {
            width,
            height,
            state,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    snake: Snake,
    board: Board,
    apple: Apple,
    pub lost: bool,
}

impl Game {
    pub fn new() -> Self {
        let width = 10;
        let height = 10;
        let snake = Snake::new();
        let apple = Apple::rand_apple(width, height);
        Game {
            board: Board::new(width, height, &snake, &apple),
            snake,
            apple,
            lost: false,
        }
    }
    pub fn check_loss(&self, next: &Point) -> bool {
        if next.x < 0 || next.x >= self.board.width {
            return true;
        }
        if next.y < 0 || next.y >= self.board.height {
            return true;
        }
        false
    }
    pub fn check_eat(&mut self, next_point: &Point) {
        if next_point.x == self.apple.location.x && next_point.y == self.apple.location.y {
            let tail = self.snake.body.back().unwrap().clone();
            self.snake.body.push_back(tail);
            self.apple = Apple::rand_apple(self.board.width, self.board.height);
        }
    }

    pub fn update(&mut self) {
        //check next square
        let next: Point;
        match self.snake.direction {
            Direction::Up => {
                next = Point {
                    x: self.snake.body.front().unwrap().x,
                    y: self.snake.body.front().unwrap().y - 1,
                }
            }
            Direction::Down => {
                next = Point {
                    x: self.snake.body.front().unwrap().x,
                    y: self.snake.body.front().unwrap().y + 1,
                }
            }
            Direction::Left => {
                next = Point {
                    x: self.snake.body.front().unwrap().x - 1,
                    y: self.snake.body.front().unwrap().y,
                }
            }
            Direction::Right => {
                next = Point {
                    x: self.snake.body.front().unwrap().x + 1,
                    y: self.snake.body.front().unwrap().y,
                }
            }
        }

        if self.check_loss(&next) {
            self.lost = true;
            return;
        }
        self.check_eat(&next);
        self.snake.slither();
        self.board.state = vec![vec![0; 10]; 10];
        //draw snake
        let snake = &self.snake.body.clone().into_iter().collect::<Vec<Point>>();
        for i in 0..snake.len() {
            self.board.state[snake[i].y as usize][snake[i].x as usize] = 1;
        }
        //check if app inside snake
        while self.board.state[self.apple.location.y as usize][self.apple.location.x as usize] == 1
        {
            self.apple = Apple::rand_apple(self.board.width, self.board.height);
        }
        self.board.state[self.apple.location.y as usize][self.apple.location.x as usize] = 2;
    }
    pub fn update_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    pub fn display(&self) {
        for row in self.board.state.iter() {
            for col in row.iter() {
                print!("{} ", col);
            }
            println!();
        }
        println!("\n");
    }
}

struct Apple {
    pub location: Point,
}
impl Apple {
    pub fn new(location: (i32, i32)) -> Self {
        Apple {
            location: Point {
                x: location.0,
                y: location.1,
            },
        }
    }
    pub fn rand_apple(width: i32, height: i32) -> Apple {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Apple {
            location: Point { x: 0, y },
        }
    }
    pub fn from_list(list: Vec<Point>) -> Apple {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..list.len());
        Apple {
            location: list[index].clone(),
        }
    }
}
