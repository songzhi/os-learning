use pendulum::Pendulum;

use super::os::Os;
use super::process::PId;
use super::statement::Statement;

pub trait Scheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: PId);
    fn switch_process(&mut self, os: &mut Os);

    fn on_tick(&mut self, os: &mut Os) {
        while let Some(pid) = os.waiting.expired_timeout() {
            self.on_process_ready(os, pid);
        }
        self.burst_process(os);
    }
    fn burst_process(&mut self, os: &mut Os) {
        let clock = os.clock;
        if let Some(process) = os.running_process() {
            if let Some(new_statement) = process.burst(clock) {
                self.run_statement(os, new_statement);
            } else if process.is_completed() {
                os.complete_process(process.id);
                self.switch_process(os);
            }
        }
    }
    fn run_statement(&mut self, os: &mut Os, statement: Statement) {
        match statement {
            Statement::CpuBound(duration) => self.run_cpu_bound_statement(os, duration),
            Statement::IoBound(duration) => self.run_io_bound_statement(os, duration),
        }
    }
    fn run_cpu_bound_statement(&mut self, os: &mut Os, duration: u64) {}
    fn run_io_bound_statement(&mut self, os: &mut Os, duration: u64) {
        let clock = os.clock;
        if let Some(process) = os.running_process() {
            process.bump_to_next(clock);
            os.await_process(process.id, duration);
        }
        self.switch_process(os);
    }
}
