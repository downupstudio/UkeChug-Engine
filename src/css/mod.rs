pub mod stylesheet;
pub mod parser;

pub use stylesheet::*;
pub use parser::Parser;

pub struct CSSParser;

impl CSSParser {
    pub fn new() -> Self {
        CSSParser
    }
    
    pub fn parse(&self, css: &str) -> Stylesheet {
        println!("  [CSS] Parsing {} bytes of CSS...", css.len());
        let mut parser = Parser::new(css.to_string());
        parser.parse_stylesheet()
    }
}