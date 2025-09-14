use llmir_diag::{Code, Diag};

#[test]
fn feature_missing_has_code_and_message() {
    let d = Diag::feature_missing("async");
    assert!(matches!(d.code, Code::EFeatureMissing));
    assert!(d.message.contains("async"));
}

#[test]
fn cap_has_code() {
    let d = Diag::cap();
    assert!(matches!(d.code, Code::ECap));
    assert!(d.message.contains("capability"));
}
