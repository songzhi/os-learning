use std::sync::Arc;

use crate::scheduling::statement::Statement;

use super::job::Job;
use super::TICK;

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
}

#[derive(Debug, Clone)]
pub struct Process {
    id: usize,
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
    pub fn burst(&mut self, clock: usize) {
        if self.is_completed() {
            return;
        }
        let next_statement_index = if let Some(running_statement) = self.running_statement.as_mut()
        {
            let elapsed_time = running_statement.elapsed_time;
            let running_statement_duration =
                self.job.statements[running_statement.index].duration();
            if elapsed_time + TICK > running_statement_duration {
                Some(running_statement.index + 1)
            } else {
                running_statement.elapsed_time += TICK;
                None
            }
        } else {
            Some(0)
        };
        next_statement_index
            .filter(|&index| self.statements().get(index).is_some())
            .map(|index| {
                self.running_statement = Some(RunningStatement::new(index));
            })
            .unwrap_or_else(|| self.complete(clock));
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
