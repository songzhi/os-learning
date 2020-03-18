use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct Job {
    pub statements: Vec<Statement>,
    pub total_duration: u64,
    pub total_cpu_duration: u64,
    pub total_io_duration: u64,
    pub is_io_bound: bool,
}

impl Job {
    pub fn cpu_bound(total_duration: u64) -> Self {
        Self {
            statements: vec![Statement::cpu_bound(total_duration)],
            total_duration,
            total_cpu_duration: total_duration,
            total_io_duration: 0,
            is_io_bound: false,
        }
    }
    /// ios: I/O statements count
    pub fn io_bound(total_duration: u64, ios: u64) -> Self {
        let total_cpu_duration = total_duration * 2 / 10;
        let total_io_duration = total_duration - total_cpu_duration;
        let cpu_duration = total_cpu_duration / ios;
        let io_duration = total_io_duration / ios;
        let mut statements = vec![];
        for _ in 0..ios {
            statements.push(Statement::cpu_bound(cpu_duration));
            statements.push(Statement::io_bound(io_duration));
        }
        Self {
            statements,
            total_duration,
            total_cpu_duration,
            total_io_duration,
            is_io_bound: true,
        }
    }
}
