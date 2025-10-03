mod html;
mod css;
mod dom;
mod style;
mod layout;
mod render;

use html::HTMLParser;
use css::CSSParser;
use dom::DOMTree;
use style::StyleEngine;
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
    
    test_html_parsing();
    
    println!();
    
    test_css_parsing();
    
    println!();
    println!("Engine initialized successfully!");
    println!("========================================");
}

fn test_html_parsing() {
    println!("Testing HTML Parser:");
    println!();
    
    let test_html = "<html><head><title>UkeChug Test</title></head><body><h1>Hello World</h1><p>This is a test paragraph.</p><div><span>Nested content</span></div></body></html>";
    
    let mut html_parser = HTMLParser::new();
    let root_node = html_parser.parse(test_html);
    
    let mut dom = DOMTree::new();
    dom.build(root_node);
    
    dom.print_tree();
    
    println!();
    println!("✓ HTML parsing complete!");
}

fn test_css_parsing() {
    println!("Testing CSS Parser:");
    println!();
    
    let test_css = "h1 { color: #ff0000; font-size: 24px; } p { color: #333333; font-size: 16px; margin: 10px; } .highlight { background: #ffff00; } #main { width: 800px; }";
    
    let css_parser = CSSParser::new();
    let stylesheet = css_parser.parse(test_css);
    
    println!("  [CSS] Parsed {} rules:", stylesheet.rules.len());
    
    for (i, rule) in stylesheet.rules.iter().enumerate() {
        println!();
        println!("  Rule {}:", i + 1);
        
        print!("    Selectors: ");
        for (j, selector) in rule.selectors.iter().enumerate() {
            if j > 0 { print!(", "); }
            match selector {
                css::Selector::Simple(s) => {
                    if let Some(ref tag) = s.tag_name {
                        print!("{}", tag);
                    }
                    if let Some(ref id) = s.id {
                        print!("#{}", id);
                    }
                    for class in &s.classes {
                        print!(".{}", class);
                    }
                }
            }
        }
        println!();
        
        println!("    Declarations:");
        for decl in &rule.declarations {
            println!("      {}: {:?}", decl.name, decl.value);
        }
    }
    
    println!();
    println!("✓ CSS parsing complete!");
}