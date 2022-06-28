use crate::core::font::FontManager;
use crate::core::Game;
use crate::core::GameState;

use sdl2::render::WindowCanvas;

type DisplayError = Box<dyn ::std::error::Error>;
type DisplayResult = Result<(), DisplayError>;

pub fn render(game: &Game, canvas: &mut WindowCanvas, font_mgr: &FontManager) -> DisplayResult {
    match &game.state {
        GameState::MainMenu(_) => main_menu::render(game, canvas, font_mgr)?,
        GameState::Playing => playing::render(canvas)?,
    }

    Ok(())
}

mod main_menu {
    use super::DisplayResult;

    use crate::core::font::FontManager;
    use crate::core::Game;

    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    pub fn render(game: &Game, canvas: &mut WindowCanvas, font_mgr: &FontManager) -> DisplayResult {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let texture_creator = canvas.texture_creator();

        for btn in game.layouts.main_menu.rects.iter() {
            canvas.set_draw_color(Color::GREEN);
            if btn.is_focused {
                let text = font_mgr
                    .monster_bites
                    .normal(32)?
                    .render(&btn.text)
                    .blended(Color::BLACK)?;

                let texture = texture_creator.create_texture_from_surface(&text)?;

                canvas.fill_rect(btn.outline)?;
                canvas.copy(&texture, None, btn.label)?;
            } else {
                let text = font_mgr
                    .monster_bites
                    .normal(32)?
                    .render(&btn.text)
                    .blended(Color::GREEN)?;

                let texture = texture_creator.create_texture_from_surface(&text)?;

                canvas.draw_rect(btn.outline)?;
                canvas.copy(&texture, None, btn.label)?;
            }
        }

        canvas.present();
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
