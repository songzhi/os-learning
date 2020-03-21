//! Shortest Remaining Job First
//!
use std::cmp::Reverse;

use keyed_priority_queue::KeyedPriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// It is preemptive mode of SJF algorithm in which jobs are schedule according to shortest remaining time.
#[derive(Default, Clone)]
pub struct ShortestRemainingJobFirstScheduler {
    ready_queue: KeyedPriorityQueue<PId, Reverse<u64>>,
}

impl ShortestRemainingJobFirstScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for ShortestRemainingJobFirstScheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: usize) {
        if let Some(process) = os.get_process(pid) {
            self.ready_queue
                .push(pid, Reverse(process.remaining_time()));
        }
    }

    fn switch_process(&mut self, os: &mut Os) {
        os.switch_process(self.ready_queue.pop().map(|(pid, _)| pid));
    }

    fn desc(&self) -> &'static str {
        "Shortest Remaining Job First; Preemptive; for Job"
    }

    fn on_process_burst(&mut self, os: &mut Os, pid: PId) {
        let process_remaining_time = os.get_process(pid).map(|p| p.remaining_time()).unwrap_or(0);
        if self
            .ready_queue
            .peek()
            .map_or(false, |(_, top_remaining_time)| {
                top_remaining_time.gt(&&Reverse(process_remaining_time))
            })
        {
            self.switch_process(os);
            self.ready_queue.push(pid, Reverse(process_remaining_time));
        }
    }
}
