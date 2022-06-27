pub enum Command {
    Quit,
}

pub enum GameState {
    Initial,
}

#[derive(Default)]
pub struct Game {
    pub state: GameState,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Initial
    }
}
