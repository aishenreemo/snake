use crate::core::Command;
use crate::core::Game;

type AmendError = Box<dyn ::std::error::Error>;
type AmendResult = Result<(), AmendError>;

pub fn update(
    _game: &mut Game, 
    commands: Vec<Command>
) -> AmendResult {
    for cmd in commands.into_iter() {
        match cmd {
            Command::Quit => std::process::exit(0),
        }
    }

    Ok(())
}
