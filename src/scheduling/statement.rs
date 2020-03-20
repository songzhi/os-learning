#[derive(Debug, Copy, Clone)]
pub enum Statement {
    CpuBound(u64),
    IoBound(u64),
}

impl Statement {
    pub fn cpu_bound(duration: u64) -> Self {
        Statement::CpuBound(duration)
    }
    pub fn io_bound(duration: u64) -> Self {
        Statement::IoBound(duration)
    }
    pub fn is_cpu_bound(&self) -> bool {
        matches!(self, Statement::CpuBound(_))
    }
    pub fn is_io_bound(&self) -> bool {
        matches!(self, Statement::IoBound(_))
    }
    pub fn duration(&self) -> u64 {
        match self {
            Statement::CpuBound(duration) => *duration,
            Statement::IoBound(duration) => *duration,
        }
    }
}
