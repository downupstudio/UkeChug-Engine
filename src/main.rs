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
use clap::Parser;

#[derive(Parser)]
#[command(name = "UkeChug Browser Engine")]
#[command(about = "A browser engine that renders HTML/CSS to PNG images", long_about = None)]
struct Args {
    #[arg(help = "HTML file to render")]
    html_file: Option<String>,
    
    #[arg(help = "CSS file to apply")]
    css_file: Option<String>,
    
    #[arg(short, long, default_value = "output.png", help = "Output PNG file")]
    output: String,
    
    #[arg(short, long, default_value = "800", help = "Image width in pixels")]
    width: u32,
    
    #[arg(short = 'H', long, default_value = "600", help = "Image height in pixels")]
    height: u32,
}

fn main() {
    let args = Args::parse();
    
    println!("========================================");
    println!("  Welcome to UkeChug Browser Engine!");
    println!("========================================");
    println!();
    println!("Version: 0.1.0");
    println!();
    
    let html_file = args.html_file.unwrap_or_else(|| "test.html".to_string());
    let css_file = args.css_file.unwrap_or_else(|| "test.css".to_string());
    
    render_from_files(&html_file, &css_file, &args.output, args.width, args.height);
    
    println!();
    println!("========================================");
}

fn render_from_files(html_file: &str, css_file: &str, output_file: &str, width: u32, height: u32) {
    println!("Loading files:");
    println!("  HTML: {}", html_file);
    println!("  CSS: {}", css_file);
    println!("  Output: {}", output_file);
    println!("  Size: {}x{}", width, height);
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
    viewport.content.width = width as f32;
    viewport.content.height = height as f32;
    
    let layout_root = layout_tree(&styled_root, viewport);
    
    println!("  [Render] Rendering to image...");
    let mut image_renderer = ImageRenderer::new(width, height);
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