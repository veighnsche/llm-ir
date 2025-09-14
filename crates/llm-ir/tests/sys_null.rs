use llmir_sys::*;
use llmir_diag::Code;

#[test]
fn null_fs_returns_e_cap() {
    let fs = NullFs::default();
    let err = fs.read("/nope").unwrap_err();
    assert!(matches!(err.code, Code::ECap));
}

#[test]
fn null_env_returns_e_cap() {
    let e = NullEnv::default();
    let err = e.get("X").unwrap_err();
    assert!(matches!(err.code, Code::ECap));
}
