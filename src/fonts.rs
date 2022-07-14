extern crate sdl2;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

pub fn init(ttf_context: &Sdl2TtfContext) -> FontManager {
    FontManager::init(ttf_context)
}

pub struct FontManager<'a> {
    path: Option<String>,
    font_size: Option<u16>,
    ttf_context: &'a Sdl2TtfContext,
}

impl<'a> FontManager<'a> {
    pub fn init(sdl2_ttf_context: &'a Sdl2TtfContext) -> Self {
        Self {
            path: None,
            font_size: None,
            ttf_context: sdl2_ttf_context,
        }
    }

    pub fn size(&mut self, s: u16) -> &mut Self {
        self.font_size = Some(s);
        self
    }

    pub fn path(&mut self, p: String) -> &mut Self {
        self.path = Some(p);
        self
    }

    pub fn load(&mut self) -> Result<Font, String> {
        if self.path.is_none() {
            return Err("Font could not be loaded without path.".to_owned());
        }

        if self.font_size.is_none() {
            return Err("Font could not be loaded without font_size.".to_owned());
        }

        self.ttf_context
            .load_font(self.path.take().unwrap(), self.font_size.take().unwrap())
    }
}
