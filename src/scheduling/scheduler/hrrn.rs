//! Highest Response Ratio Next
use keyed_priority_queue::KeyedPriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// In this scheduling, processes with highest response ratio is scheduled.
/// This algorithm avoids starvation.
/// Mode: Non-Preemptive
/// `Response Ratio = (Waiting Time + Burst time) / Burst time`
#[derive(Default, Clone)]
pub struct HighestResponseRatioNextScheduler {
    ready_queue: KeyedPriorityQueue<PId, u64>
}

impl HighestResponseRatioNextScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for HighestResponseRatioNextScheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: usize) {
        if let Some(process) = os.get_process(pid) {
            self.ready_queue.push(pid, process.job.response_ratio());
        }
    }

    fn switch_process(&mut self, os: &mut Os) {
        let pid = self.ready_queue.pop().map(|(pid, _)| pid);
        os.switch_process(pid);
    }

    fn desc(&self) -> &'static str {
        "Highest Response Ratio Next"
    }
}
