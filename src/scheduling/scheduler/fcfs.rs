use std::collections::VecDeque;

use crate::scheduling::{
    os::Os,
    process::PId,
    scheduler::Scheduler,
};

#[derive(Default, Clone)]
pub struct FCFSScheduler {
    ready_queue: VecDeque<PId>
}

impl FCFSScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for FCFSScheduler {
    fn on_process_ready(&mut self, _os: &mut Os, pid: usize) {
        self.ready_queue.push_back(pid);
    }
    fn switch_process(&mut self, os: &mut Os) {
        let pid = self.ready_queue.pop_front();
        os.switch_process(pid);
    }

    fn desc(&self) -> &'static str {
        "First Come First Serve (FCFS)"
    }
}