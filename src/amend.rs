use crate::core::Command;
use crate::core::Game;
use crate::core::GameState;

type AmendError = Box<dyn ::std::error::Error>;
type AmendResult = Result<(), AmendError>;

pub fn update(game: &mut Game, commands: Vec<Command>) -> AmendResult {
    for cmd in commands.into_iter() {
        match cmd {
            Command::Quit => std::process::exit(0),
            Command::Play => game.state = GameState::Playing,
        }
    }

    Ok(())
}
