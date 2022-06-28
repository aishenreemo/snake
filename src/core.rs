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
    // i hate naming these things
    Play,
    Quit,
}

#[derive(Default)]
pub struct Game {
    pub state: GameState,
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
