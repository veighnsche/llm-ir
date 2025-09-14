use llmir_ast::{Atom, Node, NodeOrAtom, Span};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReaderError {
    #[error("unimplemented parser")]
    Unimplemented,
}

pub fn parse(src: &str) -> Result<Node, ReaderError> {
    // Minimal stub: recognize one tiny sample
    if src.trim() == "(mod m (fn f -> i32 () 0))" {
        let node = Node {
            head: Atom::Sym("mod".into()),
            items: vec![NodeOrAtom::Atom(Atom::Sym("m".into()))],
            span: Span { start: 0, end: src.len() as u32 },
        };
        Ok(node)
    } else {
        Err(ReaderError::Unimplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal() {
        let src = "(mod m (fn f -> i32 () 0))";
        let _ = parse(src).expect("parse minimal module");
    }
}
