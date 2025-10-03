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

fn main() {
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!("Status: Initializing...");
    println!();
    
    test_enhanced_rendering();
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn test_enhanced_rendering() {
    println!("Testing Enhanced CSS Rendering:");
    println!();
    
    let test_html = "<html><body><div><h1>Welcome to UkeChug!</h1></div><div><p>This is a paragraph with styled text.</p></div><div><span>Footer text here</span></div></body></html>";
    
    let test_css = "html { display: block; width: 780px; } body { display: block; width: 700px; background-color: #f5f5f5; } div { display: block; width: 600px; background-color: white; margin: 15px; padding: 20px; border-width: 2px; border-color: #333333; } h1 { display: block; color: #0066cc; font-size: 32px; margin: 10px; } p { display: block; color: #333333; font-size: 18px; margin: 10px; } span { display: block; color: #666666; font-size: 14px; margin: 10px; }";
    
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
    
    println!("  [Render] Rendering to image with enhanced CSS...");
    
    let mut image_renderer = ImageRenderer::new(800, 600);
    image_renderer.render(&layout_root);
    
    match image_renderer.save("output.png") {
        Ok(_) => println!("  [Render] Image saved to output.png"),
        Err(e) => println!("  [Render] Error saving image: {}", e),
    }
    
    println!();
    println!("✓ Enhanced rendering complete!");
    println!();
    println!("Features demonstrated:");
    println!("  - Custom text colors (blue, gray)");
    println!("  - Background colors (white boxes on gray)");
    println!("  - Border colors and widths (dark borders)");
    println!("  - Multiple font sizes (32px, 18px, 14px)");
}