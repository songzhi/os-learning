use std::sync::Arc;

use crate::scheduling::{Job, TICK};
use crate::scheduling::statement::Statement;

pub type PId = usize;

#[derive(Debug, Copy, Clone)]
pub struct RunningStatement {
    index: usize,
    elapsed_time: u64,
}

impl RunningStatement {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            elapsed_time: 0,
        }
    }
    pub fn elapsed(self, elapsed_duration: u64) -> Self {
        Self {
            index: self.index,
            elapsed_time: self.elapsed_time + elapsed_duration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: PId,
    job: Arc<Job>,
    arrival_time: u64,
    completion_time: u64,
    burst_time: u64,
    running_statement: Option<RunningStatement>,
}

impl Process {
    pub fn new(id: usize, job: Arc<Job>, arrival_time: u64) -> Self {
        Self {
            id,
            job,
            arrival_time,
            completion_time: arrival_time,
            burst_time: 0,
            running_statement: None,
        }
    }
    pub fn complete(&mut self, completion_time: u64) {
        self.completion_time = completion_time - TICK; // error-fixing
        self.running_statement.take();
    }
    /// returns: new running statement
    pub fn burst(&mut self, clock: u64) -> Option<Statement> {
        if self.is_completed() {
            return None;
        }
        let (running_statement, statement_if_new) = self
            .running_statement
            .take()
            .map(|running_statement| {
                let elapsed_time = running_statement.elapsed_time;
                let running_statement_duration =
                    self.job.statements[running_statement.index].duration();
                if elapsed_time + TICK >= running_statement_duration {
                    let next_statement_index = running_statement.index + 1;
                    let next_statement = self
                        .statements()
                        .get(next_statement_index)
                        .map(|_| RunningStatement::new(next_statement_index));
                    (next_statement, next_statement)
                } else {
                    (Some(running_statement.elapsed(TICK)), None)
                }
            })
            .unwrap_or_else(|| {
                if self.statements().is_empty() {
                    (None, None)
                } else {
                    let next_statement = Some(RunningStatement::new(0));
                    (next_statement, next_statement)
                }
            });
        if running_statement.is_none() {
            self.complete(clock);
        }
        self.burst_time += TICK;
        self.running_statement = running_statement;
        statement_if_new.map(|s| self.statements()[s.index])
    }
    /// bump to next statement without incrementing burst time
    pub fn bump_to_next(&mut self, clock: u64) -> Option<Statement> {
        let running_statement = self.running_statement.take();
        let next_statement_index = running_statement.map(|s| s.index + 1).unwrap_or(0);
        let next_statement = self.statements().get(next_statement_index).copied();
        if next_statement.is_none() {
            if let Some(running_statement) = running_statement {
                self.complete(clock + self.statements()[running_statement.index].duration());
            } else {
                self.complete(clock);
            }
        } else {
            self.running_statement = Some(RunningStatement::new(next_statement_index));
        }
        next_statement
    }
}

impl Process {
    pub fn is_not_started(&self) -> bool {
        self.completion_time == self.arrival_time
    }
    pub fn is_running(&self) -> bool {
        self.running_statement.is_some()
    }
    pub fn is_completed(&self) -> bool {
        self.completion_time != self.arrival_time
    }
    pub fn is_io_bound(&self) -> bool {
        self.job.is_io_bound
    }
    /// Time at which the process arrives in the ready queue.
    pub fn arrival_time(&self) -> u64 {
        self.arrival_time
    }
    /// Time at which process completes its execution.
    pub fn completion_time(&self) -> u64 {
        self.completion_time
    }
    /// Time required by a process for CPU execution.
    pub fn burst_time(&self) -> u64 {
        self.burst_time
    }
    /// Time Difference between completion time and arrival time.
    pub fn turn_around_time(&self) -> u64 {
        self.completion_time - self.arrival_time
    }
    /// turn around time divides burst time
    pub fn weighted_turn_around_time(&self) -> u64 {
        self.turn_around_time()
            .checked_div(self.burst_time)
            .unwrap_or(0)
    }
    /// Time Difference between turn around time and burst time.
    pub fn waiting_time(&self) -> u64 {
        self.turn_around_time().saturating_sub(self.burst_time)
    }
    pub fn statements(&self) -> &Vec<Statement> {
        self.job.statements.as_ref()
    }
    pub fn table_header() -> prettytable::Row {
        row![
            Fgb =>
            "PId",
            "Job Type",
            "Total Duration",
            "Total I/O Duration",
            "Arrival",
            "Completion",
            "Burst",
            "Waiting",
            "Turn Around",
            "Weighted Turn Around"
        ]
    }
    pub fn table_row(&self) -> prettytable::Row {
        row![
            self.id,
            self.job.type_hint(),
            self.job.total_duration,
            self.job.total_io_duration,
            self.arrival_time,
            self.completion_time,
            self.burst_time,
            self.waiting_time(),
            self.turn_around_time(),
            self.weighted_turn_around_time()
        ]
    }
}
