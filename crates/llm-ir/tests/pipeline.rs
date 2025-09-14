#[test]
fn pipeline_smoke() {
    let src = "(mod m (fn f -> i32 () 0))";
    let node = llmir_reader::parse(src).expect("parse");
    llmir_schema::check_shapes(&node).expect("shapes");
    llmir_types::typecheck(&node).expect("types");
    let bc = llmir_lower::lower(&node).expect("lower");
    let rc = llmir_vm::run(&bc);
    assert_eq!(rc, 0);
}
