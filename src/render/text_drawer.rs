use image::{RgbaImage, Rgba};
use ab_glyph::{FontRef, PxScale, Font, ScaleFont};
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
        max_width: f32,
    ) {
        let scale = PxScale::from(size);
        let scaled_font = self.font.as_scaled(scale);
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_y = y;
        let line_height = (size * 1.5) as i32;
        
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            
            let width = self.measure_text(&test_line, &scaled_font);
            
            if width > max_width && !current_line.is_empty() {
                draw_text_mut(image, color, x, current_y, scale, &self.font, &current_line);
                current_line = word.to_string();
                current_y += line_height;
            } else {
                current_line = test_line;
            }
        }
        
        if !current_line.is_empty() {
            draw_text_mut(image, color, x, current_y, scale, &self.font, &current_line);
        }
    }
    
    fn measure_text(&self, text: &str, scaled_font: &ab_glyph::PxScaleFont<&FontRef>) -> f32 {
        let mut width = 0.0;
        for ch in text.chars() {
            let glyph = scaled_font.scaled_glyph(ch);
            width += scaled_font.h_advance(glyph.id);
        }
        width
    }
    
    pub fn calculate_text_height(&self, text: &str, size: f32, max_width: f32) -> f32 {
        let scale = PxScale::from(size);
        let scaled_font = self.font.as_scaled(scale);
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_count = 0;
        let line_height = size * 1.5;
        
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            
            let width = self.measure_text(&test_line, &scaled_font);
            
            if width > max_width && !current_line.is_empty() {
                line_count += 1;
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }
        
        if !current_line.is_empty() {
            line_count += 1;
        }
        
        line_count as f32 * line_height
    }
}