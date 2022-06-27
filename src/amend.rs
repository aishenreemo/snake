use crate::core::Command;
type AmendError = Box<dyn ::std::error::Error>;

pub fn update(commands: Vec<Command>) -> Result<(), AmendError> {
    for cmd in commands.into_iter() {
        match cmd {
            Command::Quit => std::process::exit(0),
        }
    }

    Ok(())
}
