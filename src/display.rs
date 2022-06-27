use crate::core::Game;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

type DisplayError = Box<dyn ::std::error::Error>;
type DisplayResult = Result<(), DisplayError>;

pub fn render(_game: &Game, canvas: &mut WindowCanvas) -> DisplayResult {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    Ok(())
}
