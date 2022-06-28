use crate::core::FontManager;
use crate::core::Game;
use crate::core::GameState;

use sdl2::render::WindowCanvas;

type DisplayError = Box<dyn ::std::error::Error>;
type DisplayResult = Result<(), DisplayError>;

pub fn render(game: &Game, canvas: &mut WindowCanvas, font_mgr: &FontManager) -> DisplayResult {
    match &game.state {
        GameState::MainMenu(focused) => main_menu::render(canvas, font_mgr, focused)?,
        GameState::Playing => playing::render(canvas)?,
    }

    Ok(())
}

mod main_menu {
    use super::DisplayResult;

    use crate::core::FontManager;
    use crate::core::MainMenuButton;

    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;

    pub fn render(
        canvas: &mut WindowCanvas,
        font_mgr: &FontManager,
        focused: &MainMenuButton,
    ) -> DisplayResult {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let texture_creator = canvas.texture_creator();
        let buttons = [MainMenuButton::Play, MainMenuButton::Quit];

        let size = canvas.output_size()?;

        let start = 0.4; // 50%
        let end = 0.6; // 80%
        let gap = end - start / (buttons.len() - 1) as f32;

        for (i, btn) in buttons.into_iter().enumerate() {
            let center = Point::new(
                (size.0 as f32 * 0.5) as i32,
                (size.1 as f32 * (start + (gap * i as f32))) as i32,
            );

            let width = (size.0 as f32 * 0.35) as u32;
            let height = (size.1 as f32 * 0.12) as u32;

            canvas.set_draw_color(Color::GREEN);
            let text_str = match btn {
                MainMenuButton::Play => "play",
                MainMenuButton::Quit => "quit",
            };
            let text_width = (size.0 as f32 * 0.20) as u32;
            let text_height = (size.1 as f32 * 0.08) as u32;

            if &btn == focused {
                let text = font_mgr
                    .monster_bites
                    .normal(32)?
                    .render(text_str)
                    .blended(Color::BLACK)?;

                let texture = texture_creator.create_texture_from_surface(&text)?;
                canvas.fill_rect(Rect::from_center(center, width, height))?;
                canvas.copy(
                    &texture,
                    None,
                    Rect::from_center(center, text_width, text_height),
                )?;
            } else {
                let text = font_mgr
                    .monster_bites
                    .normal(32)?
                    .render(text_str)
                    .blended(Color::GREEN)?;

                let texture = texture_creator.create_texture_from_surface(&text)?;
                canvas.draw_rect(Rect::from_center(center, width, height))?;
                canvas.copy(
                    &texture,
                    None,
                    Rect::from_center(center, text_width, text_height),
                )?;
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

mod utils {}
