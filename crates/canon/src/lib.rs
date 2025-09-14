use llmir_ast::{Node, NodeOrAtom, Atom};

pub fn format(node: &Node) -> String {
    // Very minimal s-expr printer: zero indent, one space between atoms
    fn atom(a: &Atom) -> String {
        match a {
            Atom::Sym(s) => s.clone(),
            Atom::Str(s) => format!("\"{}\"", s),
            Atom::Int(i) => i.to_string(),
            Atom::Bool(b) => if *b { "true".into() } else { "false".into() },
            Atom::Nil => "nil".into(),
        }
    }
    let mut out = atom(&node.head);
    for it in &node.items {
        let s = match it {
            NodeOrAtom::Atom(a) => atom(a),
            NodeOrAtom::Node(n) => format(&n),
        };
        out.push(' ');
        out.push_str(&s);
    }
    format!("({})", out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use llmir_ast::{Span};

    #[test]
    fn golden_min() {
        let n = Node { head: Atom::Sym("ok".into()), items: vec![NodeOrAtom::Atom(Atom::Int(0))], span: Span { start: 0, end: 0 } };
        assert_eq!(format(&n), "(ok 0)");
    }
}
