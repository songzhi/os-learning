use std::sync::Arc;

use crate::scheduling::statement::Statement;

use super::job::Job;
use super::TICK;

pub type PId = usize;

#[derive(Debug, Copy, Clone)]
pub struct RunningStatement {
    index: usize,
    elapsed_time: usize,
}

impl RunningStatement {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            elapsed_time: 0,
        }
    }
    pub fn elapsed(self, elapsed_duration: usize) -> Self {
        Self {
            index: self.index,
            elapsed_time: self.elapsed_time + elapsed_duration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    id: PId,
    job: Arc<Job>,
    arrival_time: usize,
    completion_time: usize,
    burst_time: usize,
    running_statement: Option<RunningStatement>,
}

impl Process {
    pub fn new(id: usize, job: Arc<Job>, arrival_time: usize) -> Self {
        Self {
            id,
            job,
            arrival_time,
            completion_time: arrival_time,
            burst_time: 0,
            running_statement: None,
        }
    }
    pub fn complete(&mut self, completion_time: usize) {
        self.completion_time = completion_time;
        self.running_statement.take();
    }
    /// returns: new running statement
    pub fn burst(&mut self, clock: usize) -> Option<Statement> {
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
                if elapsed_time + TICK > running_statement_duration {
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
        self.running_statement = running_statement;
        statement_if_new.map(|s| self.statements()[s.index])
    }
}

impl Process {
    pub fn is_not_started(&self) -> bool {
        self.completion_time - self.arrival_time == 0
    }
    pub fn is_running(&self) -> bool {
        self.running_statement.is_some()
    }
    pub fn is_completed(&self) -> bool {
        self.completion_time - self.arrival_time != 0
    }
    pub fn is_io_bound(&self) -> bool {
        self.job.is_io_bound
    }
    /// Time at which the process arrives in the ready queue.
    pub fn arrival_time(&self) -> usize {
        self.arrival_time
    }
    /// Time at which process completes its execution.
    pub fn completion_time(&self) -> usize {
        self.completion_time
    }
    /// Time required by a process for CPU execution.
    pub fn burst_time(&self) -> usize {
        self.burst_time
    }
    /// Time Difference between completion time and arrival time.
    pub fn turn_around_time(&self) -> usize {
        self.completion_time - self.arrival_time
    }
    /// turn around time divides burst time
    pub fn weighted_turn_around_time(&self) -> usize {
        self.turn_around_time() / self.burst_time
    }
    /// Time Difference between turn around time and burst time.
    pub fn waiting_time(&self) -> usize {
        self.turn_around_time() - self.burst_time
    }
    pub fn statements(&self) -> &Vec<Statement> {
        self.job.statements.as_ref()
    }
}
