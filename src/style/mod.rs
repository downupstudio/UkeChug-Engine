pub mod styled_node;

pub use styled_node::{StyledNode, PropertyMap, Display, style_tree};

pub struct StyleEngine;

impl StyleEngine {
    pub fn new() -> Self {
        println!("  [Style] Initializing style engine...");
        StyleEngine
    }
    
    pub fn apply_styles(&self) {
        println!("  [Style] Applying styles to elements...");
    }
}