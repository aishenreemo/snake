use crate::core::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// use sdl2::keyboard::Keycode;
type ListenerError = Box<dyn ::std::error::Error>;
type ListenerResult = Result<(), ListenerError>;

// triggers when you release a key
fn listen_key_up(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> ListenerResult {
    if let Some(Keycode::Escape) = keycode {
        commands.push(Command::Quit);
    }

    // match keycode {
    //     Some(Keycode::Escape) => commands.push(Command::Quit),
    //     _ => (),
    // }
    Ok(())
}

pub fn handle_event(commands: &mut Vec<Command>, event: Event) -> ListenerResult {
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyDown { keycode, .. } => listen_key_up(commands, keycode)?,
        _ => (),
    }

    Ok(())
}
