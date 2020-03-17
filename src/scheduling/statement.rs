#[derive(Debug, Copy, Clone)]
pub enum Statement {
    CpuBound(usize),
    IoBound(usize),
}

impl Statement {
    pub fn cpu_bound(duration: usize) -> Self {
        Statement::CpuBound(duration)
    }
    pub fn io_bound(duration: usize) -> Self {
        Statement::IoBound(duration)
    }
    pub fn is_cpu_bound(&self) -> bool {
        matches!(self, Statement::CpuBound(_))
    }
    pub fn is_io_bound(&self) -> bool {
        matches!(self, Statement::IoBound(_))
    }
    pub fn duration(&self) -> usize {
        match self {
            Statement::CpuBound(duration) => *duration,
            Statement::IoBound(duration) => *duration,
        }
    }
}