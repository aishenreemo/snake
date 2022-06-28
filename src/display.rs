use crate::core::Game;
use crate::core::GameState;

use sdl2::render::WindowCanvas;

type DisplayError = Box<dyn ::std::error::Error>;
type DisplayResult = Result<(), DisplayError>;

pub fn render(game: &Game, canvas: &mut WindowCanvas) -> DisplayResult {
    match &game.state {
        GameState::MainMenu(focused) => main_menu::render(canvas, focused)?,
        GameState::Playing => playing::render(canvas)?,
    }

    Ok(())
}

mod main_menu {
    use super::DisplayResult;

    use crate::core::MainMenuButton;

    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    pub fn render(canvas: &mut WindowCanvas, _focused: &MainMenuButton) -> DisplayResult {
        canvas.set_draw_color(Color::GRAY);
        canvas.clear();
        canvas.present();

        let buttons = [MainMenuButton::Play, MainMenuButton::Quit];

        for _btn in buttons.into_iter() {}

        Ok(())
    }
}

mod playing {

    use super::DisplayResult;

    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    pub fn render(canvas: &mut WindowCanvas) -> DisplayResult {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        Ok(())
    }
}
