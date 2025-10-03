use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}

impl Selector {
    pub fn specificity(&self) -> (usize, usize, usize) {
        let Selector::Simple(ref simple) = *self;
        let id = simple.id.iter().count();
        let classes = simple.classes.len();
        let tag = simple.tag_name.iter().count();
        (id, classes, tag)
    }
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn white() -> Color {
        Color::new(255, 255, 255, 255)
    }

    pub fn black() -> Color {
        Color::new(0, 0, 0, 255)
    }
}