use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tag {
    Mod,
    Fn,
    Lam,
    Let,
    Set,
    Match,
    Arm,
    Struct,
    Sum,
    Com,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Atom {
    Sym(String),
    Str(String),
    Int(i64),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeOrAtom {
    Node(Box<Node>),
    Atom(Atom),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub head: Atom,
    pub items: Vec<NodeOrAtom>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Path(pub Vec<String>);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Ok,
    Fast,
    Soft,
    Hard,
    Crash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResTy<T>(PhantomData<T>);

impl<T> ResTy<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_node() {
        let node = Node {
            head: Atom::Sym("mod".into()),
            items: vec![NodeOrAtom::Atom(Atom::Sym("m".into()))],
            span: Span { start: 0, end: 5 },
        };
        let s = serde_json::to_string(&node).unwrap();
        let de: Node = serde_json::from_str(&s).unwrap();
        assert_eq!(node, de);
    }
}
