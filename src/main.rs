use std::error::Error;
mod game;

fn main() -> Result<(), Box<dyn Error>> {
    let mut snake_game = game::SnakeGame::new()?;
    snake_game.run()?;
    Ok(())
}
