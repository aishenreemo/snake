use crate::core::Command;
use crate::core::Game;
use crate::core::GameState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// use sdl2::keyboard::Keycode;
type ListenerError = Box<dyn ::std::error::Error>;
type ListenerResult = Result<(), ListenerError>;

// handle keyboard release on initial game state
fn initial_key_up(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> ListenerResult {
    if let Some(Keycode::Escape) = keycode {
        commands.push(Command::Quit);
    }

    // match keycode {
    //     Some(Keycode::Escape) => commands.push(Command::Quit),
    //     _ => (),
    // }
    Ok(())
}

// handle event on initial game state
fn handle_event_on_initial(commands: &mut Vec<Command>, event: Event) -> ListenerResult {
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyDown { keycode, .. } => initial_key_up(commands, keycode)?,
        _ => (),
    }

    Ok(())
}

pub fn handle_event(game: &Game, commands: &mut Vec<Command>, event: Event) -> ListenerResult {
    match game.state {
        GameState::Initial => handle_event_on_initial(commands, event),
    }
}
