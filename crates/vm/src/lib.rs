use llmir_lower::Instr;

pub fn run(code: &[Instr]) -> i32 {
    // Minimal interpreter stub: Nop only
    let _ = code;
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use llmir_lower::Instr;

    #[test]
    fn smoke_ok() {
        let rc = run(&[Instr::Nop]);
        assert_eq!(rc, 0);
    }
}
