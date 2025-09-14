use llmir_ast::Node;
use llmir_diag::{Diag, ResultExt};

pub fn check_shapes(_node: &Node) -> Result<(), Diag> {
    // TODO: Implement table-driven arity/shape checks for core tags.
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn shapes_ok_stub() {
        assert!(true);
    }
}
