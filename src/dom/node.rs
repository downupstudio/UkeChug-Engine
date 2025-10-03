use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

impl Node {
    pub fn text(data: String) -> Node {
        Node {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }

    pub fn element(name: String, attrs: HashMap<String, String>, children: Vec<Node>) -> Node {
        Node {
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
            children,
        }
    }

    pub fn comment(data: String) -> Node {
        Node {
            node_type: NodeType::Comment(data),
            children: Vec::new(),
        }
    }
}

impl ElementData {
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    pub fn id(&self) -> Option<&String> {
        self.get_attribute("id")
    }

    pub fn classes(&self) -> Vec<&str> {
        match self.get_attribute("class") {
            Some(classlist) => classlist.split_whitespace().collect(),
            None => Vec::new(),
        }
    }
}