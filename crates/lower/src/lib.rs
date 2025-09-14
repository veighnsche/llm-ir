use llmir_ast::Node;
use llmir_diag::Diag;

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Nop,
}

pub struct LowerCtx;

pub fn lower(_n: &Node) -> Result<Vec<Instr>, Diag> {
    // TODO: implement lowering to bytecode
    Ok(vec![Instr::Nop])
}
