use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

type DisplayError = Box<dyn ::std::error::Error>;

pub fn render(canvas: &mut WindowCanvas) -> Result<(), DisplayError> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    Ok(())
}
