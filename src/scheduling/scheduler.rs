use pendulum::Pendulum;

pub use fcfs::FirstComeFirstServeScheduler;
pub use hrrn::HighestResponseRatioNextScheduler;
pub use ljf::LongestJobFirstScheduler;
pub use lrjf::LongestRemainingJobFirstScheduler;
pub use mfq::MultilevelFeedbackQueueScheduler;
pub use mq::MultilevelQueueScheduler;
pub use rr::RoundRobinScheduler;
pub use sjf::ShortestJobFirstScheduler;
pub use srjf::ShortestRemainingJobFirstScheduler;

use super::{Os, PId, statement::Statement};

mod fcfs;
mod hrrn;
mod ljf;
mod lrjf;
mod mfq;
mod mq;
mod rr;
mod sjf;
mod srjf;

pub trait Scheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: PId);
    fn switch_process(&mut self, os: &mut Os);
    fn desc(&self) -> &'static str;
    fn on_tick(&mut self, os: &mut Os) {
        while let Some(pid) = os.waiting.expired_timeout() {
            log::trace!("Clock[{}]: Process[{}] Ready", os.clock, pid);
            self.on_process_ready(os, pid);
        }
        self.burst_process(os);
    }
    fn burst_process(&mut self, os: &mut Os) {
        let clock = os.clock;
        if let Some((new_statement, is_completed, pid)) = os
            .running_process()
            .map(|process| (process.burst(clock), process.is_completed(), process.id))
        {
            self.on_process_burst(os, pid);
            if let Some(new_statement) = new_statement {
                log::trace!(
                    "Clock[{}]: Process[{}] New Statement::{:?}",
                    clock,
                    pid,
                    new_statement,
                );
                self.run_statement(os, new_statement, pid);
            } else if is_completed {
                log::trace!("Clock[{}]: Process[{}] Completed", clock, pid);
                os.complete_process(pid);
                if os.is_process_running(pid) {
                    self.switch_process(os);
                }
            }
        } else {
            self.switch_process(os);
        }
    }
    /// Run New Statement
    fn run_statement(&mut self, os: &mut Os, statement: Statement, pid: PId) {
        match statement {
            Statement::CpuBound(duration) => self.run_cpu_bound_statement(os, duration, pid),
            Statement::IoBound(duration) => self.run_io_bound_statement(os, duration, pid),
        }
    }
    fn run_cpu_bound_statement(&mut self, _os: &mut Os, _duration: u64, _pid: PId) {}
    fn run_io_bound_statement(&mut self, os: &mut Os, duration: u64, pid: PId) {
        let clock = os.clock;
        if let Some(pid) = os.get_mut_process(pid).map(|process| {
            if let Some(next_statement) = process.bump_to_next(clock) {
                log::trace!(
                    "Clock[{}]: Process[{}] Bump to Next Statement::{:?}",
                    clock,
                    process.id,
                    next_statement
                );
            }
            process.id
        }) {
            os.await_process(pid, duration)
        }
        if os.is_process_running(pid) {
            self.switch_process(os);
        }
    }
    /// BE CAREFUL!!!
    fn on_process_burst(&mut self, _os: &mut Os, _pid: PId) {}
}
