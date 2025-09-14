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

// Null providers (return E_CAP via trait defaults)
#[derive(Debug, Default, Clone, Copy)]
pub struct NullFs;
impl Fs for NullFs {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullProc;
impl Proc for NullProc {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullEnv;
impl Env for NullEnv {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullTime;
impl Time for NullTime {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullJson;
impl Json for NullJson {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullHash;
impl Hash for NullHash {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullHttp;
impl Http for NullHttp {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullGpu;
impl Gpu for NullGpu {}
