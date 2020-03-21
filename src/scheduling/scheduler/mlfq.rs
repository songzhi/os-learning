//! Multi-Level Feedback Queue
use std::collections::HashMap;

use indexmap::IndexSet;

use crate::scheduling::{Os, PId, Scheduler, TICK};

#[derive(Default, Clone)]
pub struct MultilevelFeedbackQueueScheduler {
    ready_queues: [IndexSet<PId>; 3],
    used_time_slice_map: HashMap<PId, u64>,
    running_process: Option<(PId, usize)>,
    time_slices: [u64; 2],
}

impl MultilevelFeedbackQueueScheduler {
    pub fn new(time_slices: [u64; 2]) -> Self {
        Self {
            time_slices,
            ..Self::default()
        }
    }
    /// 0/1/2, returns 0 if not found
    pub fn get_process_priority(&self, pid: PId) -> usize {
        self.running_process
            .and_then(|(running_pid, priority)| (running_pid == pid).then_some(priority))
            .unwrap_or_else(|| {
                self.ready_queues
                    .iter()
                    .enumerate()
                    .find_map(|(i, queue)| queue.get(&pid).and(Some(i)))
                    .unwrap_or(0)
            })
    }
    pub fn is_process_running(&self, pid: PId) -> bool {
        self.running_process
            .map_or(false, |(running_pid, _)| running_pid == pid)
    }
    pub fn downgrade_process(&mut self, pid: PId, clock: u64) {
        let priority = self.get_process_priority(pid);
        if priority >= self.ready_queues.len() - 1 {
            return;
        }
        log::trace!("Clock[{}]: Process[{}] Downgrade to Queue[{}]", clock,pid,priority+1);
        self.ready_queues[priority].remove(&pid);
        self.ready_queues[priority + 1].insert(pid);
    }
    pub fn last_priority(&self) -> usize {
        self.ready_queues.len() - 1
    }
}

impl Scheduler for MultilevelFeedbackQueueScheduler {
    fn on_process_ready(&mut self, _os: &mut Os, pid: usize) {
        self.ready_queues[0].insert(pid);
    }

    fn switch_process(&mut self, os: &mut Os) {
        if let Some((pid, priority)) = self
            .ready_queues
            .iter_mut()
            .enumerate()
            .find_map(|(priority, queue)| queue.pop().map(|pid| (pid, priority)))
        {
            self.running_process = Some((pid, priority));
            os.switch_process(Some(pid));
        } else {
            self.running_process = None;
            os.switch_process(None);
        }
    }

    fn desc(&self) -> &'static str {
        "Multilevel Feedback Queue; Preemptive; for Job or Process"
    }

    fn on_process_burst(&mut self, os: &mut Os, pid: usize) {
        let priority = self.get_process_priority(pid);
        let last_priority = self.last_priority();
        if priority >= last_priority {
            if self.ready_queues[0..last_priority]
                .iter()
                .any(|q| !q.is_empty())
            {
                self.ready_queues[last_priority].insert(pid);
                self.switch_process(os);
            }
        } else {
            let used_time_slice = self.used_time_slice_map.get(&pid).copied().unwrap_or(0);
            if used_time_slice >= self.time_slices[priority] && os.is_process_running(pid) {
                self.downgrade_process(pid, os.clock);
                self.used_time_slice_map.insert(pid, 0);
                self.switch_process(os);
            } else {
                self.used_time_slice_map.insert(pid, used_time_slice + TICK);
            }
        }
    }
}
