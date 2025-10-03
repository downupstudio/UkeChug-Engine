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
use render::RenderEngine;

fn main() {
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!("Status: Initializing...");
    println!();
    
    test_layout();
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn test_layout() {
    println!("Testing Layout Engine:");
    println!();
    
    let test_html = "<html><body><div id=\"main\"><h1>Hello World</h1><p>This is a paragraph with some text.</p><div><span>Nested content here</span></div></div></body></html>";
    
    let test_css = "body { display: block; margin: 0px; } div { display: block; padding: 10px; margin: 5px; } h1 { display: block; font-size: 24px; margin: 10px; } p { display: block; margin: 10px; padding: 5px; } span { display: inline; }";
    
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
    
    println!("  [Layout] Layout tree created!");
    println!();
    
    print_layout_tree(&layout_root, 0);
    
    println!();
    println!("✓ Layout calculation complete!");
}

fn print_layout_tree(layout_box: &layout::LayoutBox, indent: usize) {
    let indent_str = "  ".repeat(indent);
    
    let box_type_name = match layout_box.box_type {
        layout::BoxType::BlockNode(node) => {
            match node.node.node_type {
                dom::NodeType::Element(ref elem) => format!("Block <{}>", elem.tag_name),
                _ => "Block".to_string(),
            }
        }
        layout::BoxType::InlineNode(node) => {
            match node.node.node_type {
                dom::NodeType::Element(ref elem) => format!("Inline <{}>", elem.tag_name),
                _ => "Inline".to_string(),
            }
        }
        layout::BoxType::AnonymousBlock => "AnonymousBlock".to_string(),
    };
    
    let d = &layout_box.dimensions;
    println!("{}{}", indent_str, box_type_name);
    println!("{}  Content: x={:.1}, y={:.1}, w={:.1}, h={:.1}", 
        indent_str, d.content.x, d.content.y, d.content.width, d.content.height);
    println!("{}  Padding: l={:.1}, r={:.1}, t={:.1}, b={:.1}", 
        indent_str, d.padding.left, d.padding.right, d.padding.top, d.padding.bottom);
    println!("{}  Margin: l={:.1}, r={:.1}, t={:.1}, b={:.1}", 
        indent_str, d.margin.left, d.margin.right, d.margin.top, d.margin.bottom);
    
    for child in &layout_box.children {
        print_layout_tree(child, indent + 1);
    }
}