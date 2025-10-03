use crate::layout::{LayoutBox, BoxType};
use crate::dom::NodeType;

pub struct TextRenderer {
    canvas: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl TextRenderer {
    pub fn new(width: usize, height: usize) -> TextRenderer {
        let canvas = vec![vec![' '; width]; height];
        TextRenderer {
            canvas,
            width,
            height,
        }
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
        let d = layout_box.dimensions;
        let rect = d.border_box();

        let x = rect.x as usize;
        let y = rect.y as usize;
        let width = rect.width as usize;
        let height = rect.height as usize;

        for row in y..y.saturating_add(height).min(self.height) {
            for col in x..x.saturating_add(width).min(self.width) {
                if row < self.height && col < self.width {
                    self.canvas[row][col] = '░';
                }
            }
        }
    }

    fn render_borders(&mut self, layout_box: &LayoutBox) {
        let d = layout_box.dimensions;
        let rect = d.border_box();

        let x = rect.x as usize;
        let y = rect.y as usize;
        let width = rect.width as usize;
        let height = rect.height as usize;

        if width == 0 || height == 0 {
            return;
        }

        for col in x..x.saturating_add(width).min(self.width) {
            if y < self.height && col < self.width {
                self.canvas[y][col] = '─';
            }
            let bottom = y.saturating_add(height).saturating_sub(1);
            if bottom < self.height && col < self.width {
                self.canvas[bottom][col] = '─';
            }
        }

        for row in y..y.saturating_add(height).min(self.height) {
            if x < self.width && row < self.height {
                self.canvas[row][x] = '│';
            }
            let right = x.saturating_add(width).saturating_sub(1);
            if right < self.width && row < self.height {
                self.canvas[row][right] = '│';
            }
        }

        if x < self.width && y < self.height {
            self.canvas[y][x] = '┌';
        }
        let right = x.saturating_add(width).saturating_sub(1);
        if right < self.width && y < self.height {
            self.canvas[y][right] = '┐';
        }
        let bottom = y.saturating_add(height).saturating_sub(1);
        if x < self.width && bottom < self.height {
            self.canvas[bottom][x] = '└';
        }
        if right < self.width && bottom < self.height {
            self.canvas[bottom][right] = '┘';
        }
    }

    fn render_text(&mut self, layout_box: &LayoutBox) {
        if let BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) = &layout_box.box_type {
            if let NodeType::Element(elem) = &style_node.node.node_type {
                let d = layout_box.dimensions;
                let x = (d.content.x + d.padding.left + 1.0) as usize;
                let y = (d.content.y + d.padding.top + 1.0) as usize;

                if x < self.width && y < self.height {
                    let tag = &elem.tag_name;
                    for (i, ch) in tag.chars().enumerate() {
                        if x + i < self.width && y < self.height {
                            self.canvas[y][x + i] = ch;
                        }
                    }
                }
            }
        }
    }

    pub fn display(&self) {
        println!("Text Rendering Output:");
        println!();
        
        let mut max_row = 0;
        for (i, row) in self.canvas.iter().enumerate() {
            if row.iter().any(|&c| c != ' ') {
                max_row = i;
            }
        }
        
        for (i, row) in self.canvas.iter().enumerate() {
            if i > max_row + 1 {
                break;
            }
            let line: String = row.iter().collect();
            let trimmed = line.trim_end();
            println!("{}", trimmed);
        }
    }
}