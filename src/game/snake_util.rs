use std::{
    collections::VecDeque,
    error::Error,
    io::stdout,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub type BoxedError = Box<dyn Error>;
pub type ResultEmpty = Result<(), BoxedError>;

pub const READ_TIME: u64 = 100;
pub const FRUITE_CHAR: char = 'o';
pub const SNAKE_CHAR: char = '*';
pub const BORDER_CHAR: char = '#';

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

impl SnakeDirection {
    pub fn is_opposite(&self, other: SnakeDirection) -> bool {
        if (*self == SnakeDirection::Up && other == SnakeDirection::Down)
            || (*self == SnakeDirection::Down && other == SnakeDirection::Up)
            || (*self == SnakeDirection::Left && other == SnakeDirection::Right)
            || (*self == SnakeDirection::Right && other == SnakeDirection::Left)
        {
            return true;
        }

        false
    }
}

pub fn setup_terminal() -> ResultEmpty {
    stdout().queue(Clear(ClearType::All))?.queue(cursor::Hide)?;
    Ok(())
}

pub fn draw_at_position(point: &Point, value: char) -> ResultEmpty {
    let mut stdout = stdout();
    stdout
        .queue(cursor::MoveTo(point.x, point.y))?
        .queue(Print(value))?;
    Ok(())
}

pub fn get_command() -> Result<Option<KeyEvent>, BoxedError> {
    let mut cmd = Option::None;
    let duration = Duration::from_millis(READ_TIME);
    let start = Instant::now();
    if poll(duration)? {
        cmd = match read()? {
            Event::Key(k) => Some(k),
            Event::Mouse(_) => None,
            Event::Resize(_, _) => Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        };
    }
    let elapsed = start.elapsed();
    if elapsed < duration {
        thread::sleep(duration - elapsed);
    }
    Ok(cmd)
}

pub fn initialize_snake(width: u16, height: u16) -> VecDeque<Point> {
    let middle_height = height / 2;
    let middle_width = width / 2;
    (middle_height..(middle_height + 3))
        .map(|y| Point { x: middle_width, y })
        .collect()
}
