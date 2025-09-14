use llmir_ast::Node;
use llmir_diag::Diag;

pub fn typecheck(_n: &Node) -> Result<(), Diag> {
    // TODO: primitive types, res<T>, rails typing placeholders
    Ok(())
}
