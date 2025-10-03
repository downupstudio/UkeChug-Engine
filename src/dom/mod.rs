pub mod node;

pub use node::{Node, NodeType, ElementData};

pub struct DOMTree {
    pub root: Option<Node>,
}

impl DOMTree {
    pub fn new() -> Self {
        println!("  [DOM] Creating DOM tree...");
        DOMTree { root: None }
    }
    
    pub fn build(&mut self, root_node: Node) {
        println!("  [DOM] Building document tree structure...");
        self.root = Some(root_node);
    }

    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            println!("  [DOM] Tree structure:");
            self.print_node(root, 0);
        }
    }

    fn print_node(&self, node: &Node, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match &node.node_type {
            NodeType::Element(elem) => {
                println!("{}  <{}>", indent_str, elem.tag_name);
                for child in &node.children {
                    self.print_node(child, indent + 1);
                }
                println!("{}  </{}>", indent_str, elem.tag_name);
            }
            NodeType::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    println!("{}  \"{}\"", indent_str, trimmed);
                }
            }
            NodeType::Comment(comment) => {
                println!("{}  <!-- {} -->", indent_str, comment);
            }
        }
    }
}