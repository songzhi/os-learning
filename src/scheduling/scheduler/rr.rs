//! Round Robin
use std::collections::{HashMap, VecDeque};

use crate::scheduling::{Os, PId, Scheduler, TICK};

/// Each process is assigned a fixed time(Time Quantum/Time Slice) in cyclic way.
/// It is designed especially for the time-sharing system.
/// The ready queue is treated as a circular queue. The CPU scheduler goes around the ready queue,
/// allocating the CPU to each process for a time interval of up to 1-time quantum.
/// To implement Round Robin scheduling, we keep the ready queue as a FIFO queue of processes.
/// New processes are added to the tail of the ready queue.
/// The CPU scheduler picks the first process from the ready queue,
/// sets a timer to interrupt after 1-time quantum, and dispatches the process.
/// One of two things will then happen. The process may have a CPU burst of less than 1-time quantum.
/// In this case, the process itself will release the CPU voluntarily.
/// The scheduler will then proceed to the next process in the ready queue.
/// Otherwise, if the CPU burst of the currently running process is longer than 1-time quantum,
/// the timer will go off and will cause an interrupt to the operating system.
/// A context switch will be executed, and the process will be put at the tail of the ready queue.
/// The CPU scheduler will then select the next process in the ready queue.
#[derive(Default, Clone)]
pub struct RoundRobinScheduler {
    ready_queue: VecDeque<PId>,
    used_time_slice_map: HashMap<PId, u64>,
    time_slice: u64,
}

impl RoundRobinScheduler {
    pub fn new(time_slice: u64) -> Self {
        Self {
            ready_queue: VecDeque::new(),
            used_time_slice_map: HashMap::new(),
            time_slice,
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn on_process_ready(&mut self, _os: &mut Os, pid: usize) {
        self.ready_queue.push_back(pid);
    }

    fn switch_process(&mut self, os: &mut Os) {
        let pid = self.ready_queue.pop_front();
        os.switch_process(pid);
    }

    fn desc(&self) -> &'static str {
        "Round Robin; Preemptive; for Job or Process"
    }

    fn on_process_burst(&mut self, os: &mut Os, pid: PId) {
        let used_time_slice = self.used_time_slice_map.get(&pid).copied().unwrap_or(0);
        if used_time_slice >= self.time_slice && os.is_process_running(pid) {
            self.ready_queue.push_back(pid);
            self.used_time_slice_map.insert(pid, 0);
            self.switch_process(os);
        } else {
            self.used_time_slice_map.insert(pid, used_time_slice + TICK);
        }
    }
}
