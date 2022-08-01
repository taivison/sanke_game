use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    style::Print,
    terminal::{size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::{collections::VecDeque, io::{stdout, Write}};

use super::snake_util::*;

#[derive(Debug)]
pub struct SnakeGame {
    snake: VecDeque<Point>,
    fruit: Point,
    width: u16,
    height: u16,
    direction: SnakeDirection,
    score: u64,
}

impl SnakeGame {
    pub fn new() -> Result<Self, BoxedError> {
        let (width, mut height) = size()?;
        height -= 1;
        Ok(Self {
            snake: initialize_snake(width, height),
            fruit: Point { x: 0, y: 0 },
            width,
            height,
            direction: SnakeDirection::Up,
            score: 0,
        })
    }

    pub fn run(&mut self) -> ResultEmpty {
        self.init()?;
        loop {
            let command = get_command()?;
            if self.process_command(command)? {
                break;
            }

            if self.process_game()? {
                break;
            }

            stdout().flush()?;
        }

        stdout().execute(Clear(ClearType::All))?;
        Ok(())
    }

    fn init(&mut self) -> ResultEmpty {
        setup_terminal()?;
        self.draw_box()?;
        self.position_fruit()?;
        for p in self.snake.iter() {
            draw_at_position(p, SNAKE_CHAR)?;
        }
        self.write_score()?;
        stdout().flush()?;
        Ok(())
    }

    fn write_score(&self) -> ResultEmpty {
        stdout()
            .queue(cursor::MoveTo(0, self.height))?
            .queue(Print(format!("Score: {}", self.score)))?;
        Ok(())
    }

    fn process_command(&mut self, command: Option<KeyEvent>) -> Result<bool, BoxedError> {
        if let Some(k) = command {
            match k.code {
                KeyCode::Up => self.change_direction(SnakeDirection::Up),
                KeyCode::Down => self.change_direction(SnakeDirection::Down),
                KeyCode::Right => self.change_direction(SnakeDirection::Right),
                KeyCode::Left => self.change_direction(SnakeDirection::Left),
                KeyCode::Char(c) => self.process_char(c),
                KeyCode::Esc => return Ok(true),
                _ => (),
            }
        }

        Ok(false)
    }

    fn process_char(&mut self, c: char) {
        match c {
            'w' => self.change_direction(SnakeDirection::Up),
            's' => self.change_direction(SnakeDirection::Down),
            'd' => self.change_direction(SnakeDirection::Right),
            'a' => self.change_direction(SnakeDirection::Left),
            _ => (),
        }
    }

    fn change_direction(&mut self, new_direction: SnakeDirection) {
        if !self.direction.is_opposite(new_direction) {
            self.direction = new_direction;
        }
    }

    fn process_game(&mut self) -> Result<bool, BoxedError> {
        let head = self.get_next_snake_position()?;

        if let Some(c) = self.check_crash(&head) {
            self.crash(&head, c)?;
            return Ok(true);
        }

        self.snake.push_front(head);
        draw_at_position(&head, SNAKE_CHAR)?;
        if head == self.fruit {
            self.position_fruit()?;
        } else {
            let tail = self.snake.pop_back().ok_or("Snake is Empty!")?;
            draw_at_position(&tail, ' ')?;
        }

        self.score = (self.snake.len() - 3) as u64;
        self.write_score()?;
        Ok(false)
    }

    fn check_crash(&self, point: &Point) -> Option<char> {
        if self.is_border(point) {
            return Some(BORDER_CHAR);
        } else if self.snake.contains(point) {
            return Some(SNAKE_CHAR);
        }
        None
    }

    fn crash(&self, point: &Point, blink_char: char) -> ResultEmpty {
        self.write_score()?;
        stdout().queue(Print(" GAME OVER!"))?.flush()?;
        let mut current = ' ';
        loop {
            let cmd = get_command()?;
            if let Some(key) = cmd {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
            draw_at_position(&point, current)?;
            if current == ' ' {
                current = blink_char;
            } else {
                current = ' ';
            }
            stdout().flush()?;
        }

        Ok(())
    }

    fn get_next_snake_position(&self) -> Result<Point, BoxedError> {
        let head = self.snake.front().ok_or("Snake is Empty")?;
        let point = match self.direction {
            SnakeDirection::Up => Point {
                x: head.x,
                y: head.y - 1,
            },
            SnakeDirection::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            SnakeDirection::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
            SnakeDirection::Left => Point {
                x: head.x - 1,
                y: head.y,
            },
        };

        Ok(point)
    }

    fn draw_box(&self) -> ResultEmpty {
        for point in self.box_points() {
            draw_at_position(&point, BORDER_CHAR)?;
        }
        Ok(())
    }

    fn box_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.width)
            .map(|x| Point { x, y: 0 })
            .chain((0..self.height).map(|y| Point {
                x: self.width - 1,
                y,
            }))
            .chain((0..self.width).rev().map(|x| Point {
                x,
                y: self.height - 1,
            }))
            .chain((0..self.height).rev().map(|y| Point { x: 0, y }))
    }

    fn is_border(&self, point: &Point) -> bool {
        if point.x == 0 || point.x == (self.width - 1) {
            return true;
        }
        if point.y == 0 || point.y == (self.height - 1) {
            return true;
        }
        false
    }

    fn position_fruit(&mut self) -> ResultEmpty {
        let mut rng = rand::thread_rng();
        let mut point;
        loop {
            point = Point {
                x: rng.gen_range(1..(self.width - 1)),
                y: rng.gen_range(1..(self.height - 1)),
            };

            if !self.snake.contains(&point) {
                break;
            }
        }

        draw_at_position(&point, FRUITE_CHAR)?;
        self.fruit = point;

        Ok(())
    }
}
