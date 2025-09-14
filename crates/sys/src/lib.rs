use llmir_diag::{Code, Diag};

pub trait Fs { fn read(&self, _p: &str) -> Result<String, Diag> { Err(Diag::cap()) } }
pub trait Proc { fn spawn(&self, _cmd: &str, _args: &[String]) -> Result<i32, Diag> { Err(Diag::cap()) } }
pub trait Env { fn get(&self, _k: &str) -> Result<String, Diag> { Err(Diag::cap()) } }
pub trait Time { fn now_ms(&self) -> Result<u64, Diag> { Err(Diag::cap()) } }
pub trait Json { fn parse(&self, _s: &str) -> Result<serde_json::Value, Diag> { Err(Diag::cap()) } }
pub trait Hash { fn sha256(&self, _s: &str) -> Result<String, Diag> { Err(Diag::cap()) } }
pub trait Http { fn get(&self, _u: &str) -> Result<u16, Diag> { Err(Diag::cap()) } }
pub trait Gpu { fn pin(&self, _mask: &str) -> Result<(), Diag> { Err(Diag::cap()) } }

pub struct Sys<'a> {
    pub fs: Option<&'a dyn Fs>,
    pub proc_: Option<&'a dyn Proc>,
    pub env: Option<&'a dyn Env>,
    pub time: Option<&'a dyn Time>,
    pub json: Option<&'a dyn Json>,
    pub hash: Option<&'a dyn Hash>,
    pub http: Option<&'a dyn Http>,
    pub gpu: Option<&'a dyn Gpu>,
}

impl<'a> Default for Sys<'a> {
    fn default() -> Self {
        Sys { fs: None, proc_: None, env: None, time: None, json: None, hash: None, http: None, gpu: None }
    }
}
