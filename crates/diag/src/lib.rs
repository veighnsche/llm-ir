use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Code {
    EType,
    EArity,
    EFeatureMissing,
    EFeatureConflict,
    ECap,
    EComment,
}

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
pub struct Diag {
    #[diagnostic(code(llmir::code))]
    pub code: Code,
    pub message: String,
}

impl Diag {
    pub fn feature_missing(name: &'static str) -> Self {
        Self { code: Code::EFeatureMissing, message: format!("feature missing: {}", name) }
    }
    pub fn cap() -> Self { Self { code: Code::ECap, message: "capability missing".into() } }
}

pub trait ResultExt<T> {
    fn with_code(self, _c: Code) -> Result<T, Diag>;
}

impl<T, E: fmt::Display> ResultExt<T> for Result<T, E> {
    fn with_code(self, c: Code) -> Result<T, Diag> {
        self.map_err(|e| Diag { code: c, message: e.to_string() })
    }
}
