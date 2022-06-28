use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

pub struct FontManager<'ttf> {
    pub monster_bites: Fonts<'ttf>,
}

pub struct Fonts<'ttf> {
    normal_path: &'ttf str,
    italic_path: Option<&'ttf str>,
    pub ttf_context: &'ttf Sdl2TtfContext,
}

impl<'ttf> FontManager<'ttf> {
    pub fn new(ttf_context: &'ttf Sdl2TtfContext) -> Result<Self, Box<dyn ::std::error::Error>> {
        Ok(Self {
            monster_bites: Fonts {
                ttf_context,
                normal_path: "assets/fonts/Monster Bites/Monster Bites.ttf",
                italic_path: Some("assets/fonts/Monster Bites/Monster Bites Italic.ttf"),
            },
        })
    }
}

impl<'ttf> Fonts<'ttf> {
    pub fn normal(&'ttf self, size: u16) -> Result<Font<'ttf, 'static>, String> {
        self.ttf_context.load_font(self.normal_path, size)
    }

    pub fn italic(&'ttf self, size: u16) -> Result<Font<'ttf, 'static>, String> {
        if self.italic_path.is_none() {
            Err(String::from("No italic font."))
        } else {
            self.ttf_context.load_font(self.italic_path.unwrap(), size)
        }
    }
}
