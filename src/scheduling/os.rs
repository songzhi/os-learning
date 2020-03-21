use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use indexmap::IndexMap;
use pendulum::{HashedWheel, HashedWheelBuilder, Pendulum};

use crate::scheduling::{PId, Process, Scheduler, TICK};

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
    pub fn new(
        processes: IndexMap<PId, Process>,
        scheduler: Box<dyn Scheduler + Send>,
        jobs_desc: impl Into<String>,
    ) -> Self {
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
            "Clock[{}]: Process[{}] was Awaited with Timeout[{}]",
            self.clock,
            pid,
            timeout
        );
        self.waiting
            .insert_timeout(Duration::from_millis(timeout), pid)
            .expect("timer error");
    }
    #[allow(unused)]
    pub fn complete_process(&mut self, pid: PId) {
        if self.get_process(pid).is_none() {
            return;
        }
        log::trace!(
            "Clock[{}]: Process[{}] Completed",
            self.get_process(pid).unwrap().completion_time(),
            pid
        );
        self.completed_process_count += 1;
        if self.is_completed() {
            self.clock = self
                .processes
                .values()
                .map(|p| p.completion_time())
                .max()
                .unwrap_or(self.clock);
        }
    }
    pub fn is_completed(&self) -> bool {
        self.completed_process_count == self.processes.len()
    }
    pub fn switch_process(&mut self, pid: Option<PId>) {
        if self.running_process_pid == pid {
            return;
        }
        if let Some(pid) = pid {
            log::trace!(
                "Clock[{}]: Process[{}] was Switched to Run",
                self.clock,
                pid
            );
        } else {
            log::trace!("Clock[{}]: Idle", self.clock);
        }
        self.context_switch_times += 1;
        self.running_process_pid = pid;
    }
    pub fn is_process_running(&self, pid: PId) -> bool {
        self.running_process_pid
            .map_or(false, |running_pid| running_pid == pid)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct OsStats {
    pub average_waiting_time: u64,
    pub average_turn_around_time: u64,
    pub average_weighted_turn_around_time: u64,
    pub cpu_usage: u64,
    pub context_switch_times: usize,
}

impl std::ops::Add<&Self> for OsStats {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.average_waiting_time += rhs.average_waiting_time;
        self.average_turn_around_time += rhs.average_turn_around_time;
        self.average_weighted_turn_around_time += rhs.average_weighted_turn_around_time;
        self.cpu_usage += rhs.cpu_usage;
        self.context_switch_times += rhs.context_switch_times;
        self
    }
}

impl OsStats {
    pub fn average_stats(stats_list: &[Self]) -> Self {
        if stats_list.is_empty() {
            return Self::default();
        }
        let mut stats = stats_list.iter().fold(Self::default(), Add::add);
        let stats_count = stats_list.len() as u64;
        stats.average_waiting_time /= stats_count;
        stats.average_turn_around_time /= stats_count;
        stats.average_weighted_turn_around_time /= stats_count;
        stats.context_switch_times /= stats_count as usize;
        stats.cpu_usage /= stats_count;
        stats
    }
}

impl Os {
    pub fn stats(&self) -> OsStats {
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
        OsStats {
            average_waiting_time,
            average_turn_around_time,
            average_weighted_turn_around_time,
            cpu_usage,
            context_switch_times: self.context_switch_times,
        }
    }
    pub fn detailed_process_stats_table(&self) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        table.set_titles(Process::table_titles());
        crate::utils::set_table_format(&mut table);
        for p in self.processes.values() {
            table.add_row(p.table_row());
        }
        table
    }
    pub fn totalled_stats_titles() -> prettytable::Row {
        row![
            Fgb =>
            "Job",
            "Scheduler",
            "Ave Waiting",
            "Ave Turn Around",
            "Ave Wtd Turn Around",
            "CPU Usage",
            "Context Switches"
        ]
    }
    pub fn stats_row(&self, stats: OsStats, average_stats: Option<OsStats>) -> prettytable::Row {
        let OsStats {
            average_waiting_time,
            average_turn_around_time,
            average_weighted_turn_around_time,
            cpu_usage,
            context_switch_times,
        } = stats;
        let with_diff = |x: u64, ave: u64| -> String {
            let (mark, diff) = if x >= ave {
                ('+', x - ave)
            } else {
                ('-', ave - x)
            };
            format!("{}({}{}%)", x, mark, diff * 100 / ave)
        };
        if let Some(average_stats) = average_stats {
            row![
                self.jobs_desc,
                r->self.scheduler.lock().expect("lock failed").desc(),
                with_diff(average_waiting_time,average_stats.average_waiting_time),
                with_diff(average_turn_around_time,average_stats.average_turn_around_time),
                with_diff(average_weighted_turn_around_time,average_stats.average_weighted_turn_around_time),
                format!("{}%", cpu_usage),
                with_diff(context_switch_times as u64, average_stats.context_switch_times as u64)
            ]
        } else {
            row![
                self.jobs_desc,
                r->self.scheduler.lock().expect("lock failed").desc(),
                average_waiting_time,
                average_turn_around_time,
                average_weighted_turn_around_time,
                format!("{}%", cpu_usage),
                context_switch_times
            ]
        }
    }
    pub fn stats_table(&self) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        crate::utils::set_table_format(&mut table);
        table.set_titles(Self::totalled_stats_titles());
        table.add_row(self.stats_row(self.stats(), None));
        table
    }
    pub fn os_list_stats_table(os_list: &[Os]) -> prettytable::Table {
        let mut table = prettytable::Table::new();
        crate::utils::set_table_format(&mut table);
        table.set_titles(Self::totalled_stats_titles());
        let stats_list = os_list.iter().map(|os| os.stats()).collect::<Vec<_>>();
        let average_stats = OsStats::average_stats(stats_list.as_slice());
        for (stats, os) in stats_list.into_iter().zip(os_list) {
            table.add_row(os.stats_row(stats, Some(average_stats)));
        }
        table
    }
    pub fn desc(&self) -> String {
        format!(
            "Job: {}  Scheduler: {}",
            self.jobs_desc,
            self.scheduler.lock().expect("lock failed").desc()
        )
    }
}
