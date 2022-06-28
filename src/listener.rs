use crate::core::Command;
use crate::core::Game;
use crate::core::GameState;

use sdl2::event::Event;

// use sdl2::keyboard::Keycode;
type ListenerError = Box<dyn ::std::error::Error>;
type ListenerResult = Result<(), ListenerError>;

pub fn handle_event(game: &Game, commands: &mut Vec<Command>, event: Event) -> ListenerResult {
    match &game.state {
        GameState::MainMenu(focused) => main_menu::handle_event(commands, event, focused),
        GameState::Playing => playing::handle_event(commands, event),
    }
}

mod main_menu {
    use super::ListenerResult;

    use crate::core::Command;
    use crate::core::MainMenuButton;

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    pub fn handle_event(
        commands: &mut Vec<Command>,
        event: Event,
        focused: &MainMenuButton,
    ) -> ListenerResult {
        match event {
            Event::Quit { .. } => commands.push(Command::Quit),
            Event::KeyUp { keycode, .. } => on_key_up(commands, keycode, focused)?,
            _ => (),
        }

        Ok(())
    }

    fn on_key_up(
        commands: &mut Vec<Command>,
        keycode: Option<Keycode>,
        focused: &MainMenuButton,
    ) -> ListenerResult {
        match keycode {
            Some(Keycode::Escape) => commands.push(Command::Quit),
            Some(Keycode::Return) => on_enter(commands, focused)?,
            _ => (),
        }

        Ok(())
    }

    fn on_enter(commands: &mut Vec<Command>, focused: &MainMenuButton) -> ListenerResult {
        match focused {
            MainMenuButton::Play => commands.push(Command::Play),
            MainMenuButton::Quit => commands.push(Command::Quit),
        }

        Ok(())
    }
}

mod playing {
    use super::ListenerResult;

    use crate::core::Command;

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    pub fn handle_event(commands: &mut Vec<Command>, event: Event) -> ListenerResult {
        match event {
            Event::Quit { .. } => commands.push(Command::Quit),
            Event::KeyUp { keycode, .. } => on_key_up(commands, keycode)?,
            _ => (),
        }

        Ok(())
    }

    fn on_key_up(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> ListenerResult {
        match keycode {
            Some(Keycode::Escape) => commands.push(Command::Quit),
            Some(_) => (),
            None => (),
        }

        Ok(())
    }
}
