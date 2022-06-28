use crate::core::MainMenuButton;

use sdl2::rect::Point;
use sdl2::rect::Rect;

pub struct Layouts {
    pub main_menu: MainMenuLayout,
}

pub struct MainMenuLayout {
    // outline rectangle, label rectangle, is focused, label text
    pub rects: Vec<ButtonLayout>,
}

pub struct ButtonLayout {
    pub outline: Rect,
    pub label: Rect,
    pub is_focused: bool,
    pub text: String,
}

impl Layouts {
    pub fn init(size: (u32, u32)) -> Self {
        Self {
            main_menu: MainMenuLayout::init(size),
        }
    }
}

impl MainMenuLayout {
    pub fn init(size: (u32, u32)) -> Self {
        let buttons = [MainMenuButton::Play, MainMenuButton::Quit];

        let start = 0.4;
        let end = 0.6;
        let gap = end - start / (buttons.len() - 1) as f32;

        let mut rects = vec![];
        for (i, btn) in buttons.into_iter().enumerate() {
            let center = Point::new(
                (size.0 as f32 * 0.5) as i32,
                (size.1 as f32 * (start + (gap * i as f32))) as i32,
            );

            let width = (size.0 as f32 * 0.35) as u32;
            let height = (size.1 as f32 * 0.12) as u32;

            let text_width = (size.0 as f32 * 0.20) as u32;
            let text_height = (size.1 as f32 * 0.08) as u32;
            let text_str = match btn {
                MainMenuButton::Play => "play".to_owned(),
                MainMenuButton::Quit => "quit".to_owned(),
            };

            rects.push(ButtonLayout::new(
                Rect::from_center(center, width, height),
                Rect::from_center(center, text_width, text_height),
                btn == MainMenuButton::Play,
                text_str,
            ));
        }

        Self { rects }
    }
}

impl ButtonLayout {
    pub fn new(outline: Rect, label: Rect, is_focused: bool, text: String) -> Self {
        Self {
            outline,
            label,
            is_focused,
            text,
        }
    }

    pub fn toggle_focus(&mut self) {
        self.is_focused = !self.is_focused;
    }
}
