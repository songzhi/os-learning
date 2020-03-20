use std::sync::{Arc, Mutex};
use std::time::Duration;

use indexmap::IndexMap;
use pendulum::{HashedWheel, HashedWheelBuilder, Pendulum};

use super::process::{PId, Process};
use super::scheduler::Scheduler;
use super::TICK;

pub struct Os {
    pub(crate) clock: u64,
    processes: IndexMap<PId, Process>,
    pub(crate) waiting: HashedWheel<PId>,
    running_process_pid: Option<PId>,
    // thread-safe actually not needed
    scheduler: Arc<Mutex<Box<dyn Scheduler + Send>>>,
    completed_process_count: usize,
    context_switch_times: usize,
    jobs_desc: String,
}

impl Os {
    pub fn new(processes: IndexMap<PId, Process>, scheduler: Box<dyn Scheduler + Send>, jobs_desc: impl Into<String>) -> Self {
        let mut waiting = HashedWheelBuilder::default()
            .with_tick_duration(Duration::from_millis(TICK))
            .with_max_timeout(Duration::from_secs(1000))
            .build();
        for p in processes.values() {
            waiting
                .insert_timeout(Duration::from_millis(p.arrival_time()), p.id)
                .expect("timer error");
        }
        Self {
            clock: 0,
            processes,
            waiting,
            running_process_pid: None,
            scheduler: Arc::new(Mutex::new(scheduler)),
            completed_process_count: 0,
            context_switch_times: 0,
            jobs_desc: jobs_desc.into(),
        }
    }
    pub fn run(&mut self) {
        while !self.is_completed() {
            self.tick();
        }
    }
    pub fn tick(&mut self) {
        self.clock += TICK;
        self.waiting.tick();
        let scheduler = self.scheduler.clone();
        let mut scheduler = scheduler.lock().expect("lock failed");
        scheduler.on_tick(self);
    }

    pub fn running_process(&mut self) -> Option<&mut Process> {
        self.running_process_pid
            .and_then(move |pid| self.processes.get_mut(&pid))
    }
    pub fn get_process(&self, pid: PId) -> Option<&Process> {
        self.processes.get(&pid)
    }
    pub fn get_mut_process(&mut self, pid: PId) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }
    pub fn await_process(&mut self, pid: PId, timeout: u64) {
        log::trace!(
            "Clock[{}]: Await Process[{}] with Timeout[{}]",
            self.clock,
            pid,
            timeout
        );
        self.waiting
            .insert_timeout(Duration::from_millis(timeout), pid)
            .expect("timer error");
    }
    pub fn complete_process(&mut self, _pid: PId) {
        self.completed_process_count += 1;
    }
    pub fn is_completed(&self) -> bool {
        self.completed_process_count == self.processes.len()
    }
    pub fn switch_process(&mut self, pid: Option<PId>) {
        if let Some(pid) = pid {
            log::trace!("Clock[{}]: Switch to Process[{}]", self.clock, pid);
            self.context_switch_times += 1;
        } else {
            log::trace!("Clock[{}]: Idle", self.clock);
        }
        self.running_process_pid = pid;
    }
    pub fn raw_process_stats(&self) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        table.add_row(Process::table_header());
        for p in self.processes.values() {
            table.add_row(p.table_row());
        }
        table
    }
    pub fn totalled_process_stats_titles() -> prettytable::Row {
        row![
                Fgb =>
                "Job",
                "Scheduler",
                "Ave Waiting",
                "Ave Turn Around",
                "Ave Weighted Turn Around",
                "CPU Usage",
                "Context Switches"
            ]
    }
    pub fn totalled_process_stats_row(&self) -> prettytable::Row {
        let mut waiting_time_sum = 0;
        let mut turn_around_time_sum = 0;
        let mut weighted_turn_around_time_sum = 0;
        let mut burst_time_sum = 0;
        for p in self.processes.values() {
            waiting_time_sum += p.waiting_time();
            turn_around_time_sum += p.turn_around_time();
            weighted_turn_around_time_sum += p.weighted_turn_around_time();
            burst_time_sum += p.burst_time();
        }
        let process_count = self.processes.len() as u64;
        let average_waiting_time = waiting_time_sum / process_count;
        let average_turn_around_time = turn_around_time_sum / process_count;
        let average_weighted_turn_around_time = weighted_turn_around_time_sum / process_count;
        let cpu_usage = burst_time_sum * 100 / self.clock;
        row![
            self.jobs_desc,
            self.scheduler.lock().expect("lock failed").desc(),
            average_waiting_time,
            average_turn_around_time,
            average_weighted_turn_around_time,
            format!("{}%", cpu_usage),
            self.context_switch_times
        ]
    }
    pub fn totalled_process_stats(&self) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        crate::utils::set_table_format(&mut table);
        table.set_titles(Self::totalled_process_stats_titles());
        table.add_row(self.totalled_process_stats_row());
        table
    }
    pub fn os_list_totalled_process_stats(os_list: &[Os]) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        crate::utils::set_table_format(&mut table);
        table.set_titles(Self::totalled_process_stats_titles());
        for os in os_list {
            table.add_row(os.totalled_process_stats_row());
        }
        table
    }
    pub fn desc(&self) -> String {
        format!("Job: {}  Scheduler: {}", self.jobs_desc, self.scheduler.lock().expect("lock failed").desc())
    }
}
