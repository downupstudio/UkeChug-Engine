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
use layout::LayoutEngine;
use render::RenderEngine;

fn main() {
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!("Status: Initializing...");
    println!();
    
    test_style_matching();
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn test_style_matching() {
    println!("Testing Style Matching:");
    println!();
    
    let test_html = "<html><body><div id=\"main\" class=\"container\"><h1>Hello World</h1><p class=\"highlight\">This is a test.</p></div></body></html>";
    
    let test_css = "h1 { color: #ff0000; font-size: 24px; display: block; } p { color: #333333; font-size: 16px; display: block; } .highlight { background: #ffff00; } #main { width: 800px; display: block; } .container { margin: 20px; }";
    
    let mut html_parser = HTMLParser::new();
    let root_node = html_parser.parse(test_html);
    
    let css_parser = CSSParser::new();
    let stylesheet = css_parser.parse(test_css);
    
    println!("  [Style] Creating styled tree...");
    let styled_root = style_tree(&root_node, &stylesheet);
    
    println!("  [Style] Styled tree created!");
    println!();
    
    print_styled_tree(&styled_root, 0);
    
    println!();
    println!("✓ Style matching complete!");
}

fn print_styled_tree(node: &style::StyledNode, indent: usize) {
    let indent_str = "  ".repeat(indent);
    
    match node.node.node_type {
        dom::NodeType::Element(ref elem) => {
            println!("{}Element: <{}>", indent_str, elem.tag_name);
            
            if !node.specified_values.is_empty() {
                println!("{}  Styles:", indent_str);
                for (name, value) in &node.specified_values {
                    println!("{}    {}: {:?}", indent_str, name, value);
                }
            }
            
            for child in &node.children {
                print_styled_tree(child, indent + 1);
            }
        }
        dom::NodeType::Text(ref text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                println!("{}Text: \"{}\"", indent_str, trimmed);
            }
        }
        dom::NodeType::Comment(_) => {}
    }
}