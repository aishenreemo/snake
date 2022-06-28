mod layout;

pub mod font;

use layout::Layouts;

pub struct Game {
    pub state: GameState,
    pub layouts: Layouts,
}

pub enum Command {
    Play,
    Quit,
}

pub enum GameState {
    MainMenu(MainMenuButton),
    Playing,
}

#[derive(PartialEq, Eq)]
pub enum MainMenuButton {
    Play,
    Quit,
}

impl Game {
    pub fn init(size: (u32, u32)) -> Self {
        Self {
            layouts: Layouts::init(size),
            state: GameState::default(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::MainMenu(MainMenuButton::default())
    }
}

impl Default for MainMenuButton {
    fn default() -> Self {
        Self::Play
    }
}
