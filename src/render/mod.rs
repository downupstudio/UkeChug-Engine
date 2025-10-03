pub mod text_renderer;
pub mod image_renderer;
pub mod text_drawer;

pub use image_renderer::ImageRenderer;

pub struct RenderEngine;

impl RenderEngine {
    pub fn new() -> Self {
        println!("  [Render] Initializing render engine...");
        RenderEngine
    }
    
    pub fn render(&self) {
        println!("  [Render] Drawing webpage to screen...");
    }
}