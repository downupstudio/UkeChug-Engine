use crate::layout::{LayoutBox, BoxType};
use crate::dom::NodeType;
use crate::css::Value;
use crate::render::text_drawer::TextDrawer;
use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use imageproc::rect::Rect;

pub struct ImageRenderer<'a> {
    image: RgbaImage,
    text_drawer: TextDrawer<'a>,
}

impl<'a> ImageRenderer<'a> {
    pub fn new(width: u32, height: u32) -> ImageRenderer<'a> {
        let image = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));
        let text_drawer = TextDrawer::new();
        ImageRenderer { image, text_drawer }
    }

    pub fn render(&mut self, layout_root: &LayoutBox) {
        self.render_layout_box(layout_root);
    }

    fn render_layout_box(&mut self, layout_box: &LayoutBox) {
        self.render_background(layout_box);
        self.render_borders(layout_box);
        self.render_text(layout_box);

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
        let border_color = self.get_border_color(layout_box);
        let border_width = self.get_border_width(layout_box);

        if border_width == 0 {
            return;
        }

        let d = layout_box.dimensions;
        let rect = d.border_box();

        let x = rect.x as i32;
        let y = rect.y as i32;
        let width = rect.width as i32;
        let height = rect.height as i32;

        if width > 0 && height > 0 {
            for i in 0..border_width {
                let offset = i as i32;
                let adjusted_x = x + offset;
                let adjusted_y = y + offset;
                let adjusted_width = (width - offset * 2).max(1) as u32;
                let adjusted_height = (height - offset * 2).max(1) as u32;

                if adjusted_width > 0 && adjusted_height > 0 {
                    let image_rect = Rect::at(adjusted_x, adjusted_y)
                        .of_size(adjusted_width, adjusted_height);
                    draw_hollow_rect_mut(&mut self.image, image_rect, border_color);
                }
            }
        }
    }

    fn render_text(&mut self, layout_box: &LayoutBox) {
        if let BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) = &layout_box.box_type {
            if let NodeType::Element(elem) = &style_node.node.node_type {
                println!("Rendering text for <{}>", elem.tag_name);
            }
            
            for child_node in &style_node.node.children {
                if let NodeType::Text(text) = &child_node.node_type {
                    let d = layout_box.dimensions;
                    
                    println!("  Box dimensions:");
                    println!("    content: x={}, y={}, w={}, h={}", d.content.x, d.content.y, d.content.width, d.content.height);
                    println!("    padding: l={}, t={}", d.padding.left, d.padding.top);
                    
                    let x = (d.content.x + d.padding.left + 10.0) as i32;
                    let y = (d.content.y + d.padding.top + 10.0) as i32;
                    let max_width = d.content.width.max(100.0) - 20.0;
                    
                    println!("    text position: x={}, y={}, max_width={}", x, y, max_width);
                    
                    let font_size = self.get_font_size(style_node);
                    let text_color = self.get_text_color(style_node);
                    
                    self.text_drawer.draw_text(&mut self.image, text.trim(), x, y, font_size, text_color, max_width);
                }
            }
        }
    }

    fn get_background_color(&self, layout_box: &LayoutBox) -> Rgba<u8> {
        match &layout_box.box_type {
            BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) => {
                if let Some(value) = style_node.value("background-color") {
                    return self.value_to_color(value);
                }
                if let Some(value) = style_node.value("background") {
                    return self.value_to_color(value);
                }
            }
            _ => {}
        }
        Rgba([255, 255, 255, 255])
    }

    fn get_border_color(&self, layout_box: &LayoutBox) -> Rgba<u8> {
        match &layout_box.box_type {
            BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) => {
                if let Some(value) = style_node.value("border-color") {
                    return self.value_to_color(value);
                }
            }
            _ => {}
        }
        Rgba([0, 0, 0, 255])
    }

    fn get_border_width(&self, layout_box: &LayoutBox) -> u32 {
        match &layout_box.box_type {
            BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) => {
                if let Some(Value::Length(width, _)) = style_node.value("border-width") {
                    return (*width as u32).max(0);
                }
            }
            _ => {}
        }
        1
    }

    fn get_text_color(&self, style_node: &crate::style::StyledNode) -> Rgba<u8> {
        if let Some(value) = style_node.value("color") {
            return self.value_to_color(value);
        }
        Rgba([0, 0, 0, 255])
    }

    fn get_font_size(&self, style_node: &crate::style::StyledNode) -> f32 {
        if let Some(Value::Length(size, _)) = style_node.value("font-size") {
            return *size;
        }
        16.0
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
                "gray" => Rgba([128, 128, 128, 255]),
                "lightgray" => Rgba([211, 211, 211, 255]),
                "darkgray" => Rgba([169, 169, 169, 255]),
                _ => Rgba([255, 255, 255, 255]),
            },
            _ => Rgba([255, 255, 255, 255]),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.image.save(path)
    }
}