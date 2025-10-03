mod html;
mod css;
mod dom;
mod style;
mod layout;
mod render;

use html::HTMLParser;
use css::CSSParser;
use style::style_tree;
use layout::{layout_tree, Dimensions};
use render::ImageRenderer;
use std::fs;

fn main() {
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!("Status: Initializing...");
    println!();
    
    render_from_files("test.html", "test.css", "output.png");
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn render_from_files(html_file: &str, css_file: &str, output_file: &str) {
    println!("Loading files:");
    println!("  HTML: {}", html_file);
    println!("  CSS: {}", css_file);
    println!();
    
    let html_content = match fs::read_to_string(html_file) {
        Ok(content) => {
            println!("  [File] Loaded {} ({} bytes)", html_file, content.len());
            content
        }
        Err(e) => {
            println!("  [Error] Could not read {}: {}", html_file, e);
            return;
        }
    };
    
    let css_content = match fs::read_to_string(css_file) {
        Ok(content) => {
            println!("  [File] Loaded {} ({} bytes)", css_file, content.len());
            content
        }
        Err(e) => {
            println!("  [Error] Could not read {}: {}", css_file, e);
            return;
        }
    };
    
    println!();
    println!("  [HTML] Parsing HTML...");
    let mut html_parser = HTMLParser::new();
    let root_node = html_parser.parse(&html_content);
    
    println!("  [CSS] Parsing CSS...");
    let css_parser = CSSParser::new();
    let stylesheet = css_parser.parse(&css_content);
    
    println!("  [Style] Creating styled tree...");
    let styled_root = style_tree(&root_node, &stylesheet);
    
    println!("  [Layout] Creating layout tree...");
    let mut viewport: Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    
    let layout_root = layout_tree(&styled_root, viewport);
    
    println!("  [Render] Rendering to image...");
    let mut image_renderer = ImageRenderer::new(800, 600);
    image_renderer.render(&layout_root);
    
    match image_renderer.save(output_file) {
        Ok(_) => {
            println!("  [Render] Image saved to {}", output_file);
            println!();
            println!("Success! Open {} to see your rendered page.", output_file);
        }
        Err(e) => println!("  [Render] Error saving image: {}", e),
    }
}