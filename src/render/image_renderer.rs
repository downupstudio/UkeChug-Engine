use crate::layout::{LayoutBox, BoxType};
use crate::dom::NodeType;
use crate::css::Value;
use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use imageproc::rect::Rect;

pub struct ImageRenderer {
    image: RgbaImage,
}

impl ImageRenderer {
    pub fn new(width: u32, height: u32) -> ImageRenderer {
        let image = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));
        ImageRenderer { image }
    }

    pub fn render(&mut self, layout_root: &LayoutBox) {
        self.render_layout_box(layout_root);
    }

    fn render_layout_box(&mut self, layout_box: &LayoutBox) {
        self.render_background(layout_box);
        self.render_borders(layout_box);

        for child in &layout_box.children {
            self.render_layout_box(child);
        }
    }

    fn render_background(&mut self, layout_box: &LayoutBox) {
        let color = self.get_background_color(layout_box);
        
        let d = layout_box.dimensions;
        let rect = d.border_box();

        let x = rect.x as i32;
        let y = rect.y as i32;
        let width = rect.width as i32;
        let height = rect.height as i32;

        if width > 0 && height > 0 {
            let image_rect = Rect::at(x, y).of_size(width as u32, height as u32);
            draw_filled_rect_mut(&mut self.image, image_rect, color);
        }
    }

    fn render_borders(&mut self, layout_box: &LayoutBox) {
        let d = layout_box.dimensions;
        let rect = d.border_box();

        let x = rect.x as i32;
        let y = rect.y as i32;
        let width = rect.width as i32;
        let height = rect.height as i32;

        if width > 0 && height > 0 {
            let image_rect = Rect::at(x, y).of_size(width as u32, height as u32);
            let border_color = Rgba([0, 0, 0, 255]);
            draw_hollow_rect_mut(&mut self.image, image_rect, border_color);
        }
    }

    fn get_background_color(&self, layout_box: &LayoutBox) -> Rgba<u8> {
        match &layout_box.box_type {
            BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) => {
                if let Some(value) = style_node.value("background") {
                    return self.value_to_color(value);
                }
                if let Some(value) = style_node.value("background-color") {
                    return self.value_to_color(value);
                }
            }
            _ => {}
        }
        Rgba([240, 240, 240, 255])
    }

    fn value_to_color(&self, value: &Value) -> Rgba<u8> {
        match value {
            Value::Color(c) => Rgba([c.r, c.g, c.b, c.a]),
            Value::Keyword(k) => match k.as_str() {
                "white" => Rgba([255, 255, 255, 255]),
                "black" => Rgba([0, 0, 0, 255]),
                "red" => Rgba([255, 0, 0, 255]),
                "green" => Rgba([0, 255, 0, 255]),
                "blue" => Rgba([0, 0, 255, 255]),
                "yellow" => Rgba([255, 255, 0, 255]),
                _ => Rgba([240, 240, 240, 255]),
            },
            _ => Rgba([240, 240, 240, 255]),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.image.save(path)
    }
}