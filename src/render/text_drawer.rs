use image::{RgbaImage, Rgba};
use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::draw_text_mut;

pub struct TextDrawer<'a> {
    font: FontRef<'a>,
}

impl<'a> TextDrawer<'a> {
    pub fn new() -> TextDrawer<'a> {
        let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
        let font = FontRef::try_from_slice(font_data)
            .expect("Error loading font");
        TextDrawer { font }
    }

    pub fn draw_text(
        &self,
        image: &mut RgbaImage,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: Rgba<u8>,
    ) {
        let scale = PxScale::from(size);
        draw_text_mut(image, color, x, y, scale, &self.font, text);
    }
}