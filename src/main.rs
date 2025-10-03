mod html;
mod css;
mod dom;
mod style;
mod layout;
mod render;

use html::HTMLParser;
use css::CSSParser;
use dom::DOMTree;
use style::{StyleEngine, style_tree};
use layout::{LayoutEngine, layout_tree, Dimensions, Rect};
use render::{RenderEngine, TextRenderer, ImageRenderer};

fn main() {
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!("Status: Initializing...");
    println!();
    
    test_image_rendering();
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn test_image_rendering() {
    println!("Testing Image Rendering:");
    println!();
    
    let test_html = "<html><body><div><h1>Header</h1></div><div><p>Paragraph</p></div></body></html>";
    
    let test_css = "html { display: block; width: 780px; } body { display: block; width: 700px; background: white; } div { display: block; width: 600px; height: 100px; background: #e0e0e0; margin: 10px; } h1 { display: block; width: 500px; height: 50px; background: #ffcccc; margin: 10px; } p { display: block; width: 500px; height: 50px; background: #ccccff; margin: 10px; }";
    
    let mut html_parser = HTMLParser::new();
    let root_node = html_parser.parse(test_html);
    
    let css_parser = CSSParser::new();
    let stylesheet = css_parser.parse(test_css);
    
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
    
    match image_renderer.save("output.png") {
        Ok(_) => println!("  [Render] Image saved to output.png"),
        Err(e) => println!("  [Render] Error saving image: {}", e),
    }
    
    println!();
    println!("✓ Image rendering complete!");
}