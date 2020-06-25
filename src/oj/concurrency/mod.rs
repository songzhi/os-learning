use std::collections::{HashMap, VecDeque};
use std::vec::IntoIter;

use either::Either;

pub use parser::parse;

mod parser;

type VarOrNum = Either<String, i32>;

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(String, VarOrNum),
    AddAssignment(String, VarOrNum),
    Print(String),
    Lock,
    UnLock,
    Yield,
    End,
}

pub struct Process {
    pid: usize,
    statements: IntoIter<Statement>,
    environment: HashMap<String, i32>,
    time_slice: usize,
}

impl Process {
    fn new(pid: usize, statements: IntoIter<Statement>, time_slice: usize) -> Self {
        Self {
            pid,
            statements,
            environment: Default::default(),
            time_slice,
        }
    }
    fn next_statement(&mut self) -> Option<Statement> {
        self.statements.next()
    }
    fn assign(&mut self, var: String, right: VarOrNum) {
        let val = match right {
            Either::Left(var) => self.environment.get(&var).cloned().unwrap_or(0),
            Either::Right(v) => v,
        };
        self.environment.insert(var, val);
    }
    fn add_assign(&mut self, var: String, right: VarOrNum) {
        let val = self.environment.get(&var).cloned().unwrap_or(0)
            + match right {
            Either::Left(var) => self.environment.get(&var).cloned().unwrap_or(0),
            Either::Right(v) => v,
        };
        self.environment.insert(var, val);
    }
    fn print(&self, var: String) {
        println!(
            "{}: {}",
            self.pid,
            self.environment.get(&var).cloned().unwrap_or(0)
        );
    }
}

pub struct Os {
    stmt_exec_time_cfg: [usize; 7],
    time_slice: usize,
    ready_queue: VecDeque<Process>,
    blocking_queue: VecDeque<Process>,
    is_locked: bool,
}

impl Os {
    pub fn new(
        stmt_exec_time_cfg: [usize; 7],
        time_slice: usize,
        statements: Vec<Statement>,
    ) -> Self {
        let ready_queue = statements
            .split(|s| matches!(s, Statement::End))
            .map(|s| s.to_vec().into_iter())
            .enumerate()
            .map(|(pid, statements)| Process::new(pid + 1, statements, time_slice))
            .collect();
        Self {
            stmt_exec_time_cfg,
            time_slice,
            ready_queue,
            blocking_queue: Default::default(),
            is_locked: false,
        }
    }

    pub fn run(mut self) {
        while let Some(mut running_process) = self.ready_queue.pop_front() {
            while let Some(stmt) = running_process.next_statement() {
                running_process.time_slice = running_process
                    .time_slice
                    .saturating_sub(self.statement_execution_time(&stmt));
                match stmt {
                    Statement::Assignment(var, right) => running_process.assign(var, right),
                    Statement::AddAssignment(var, right) => running_process.add_assign(var, right),
                    Statement::Print(var) => running_process.print(var),
                    Statement::Lock => {
                        if self.is_locked {
                            running_process.time_slice = self.time_slice;
                            self.blocking_queue.push_back(running_process);
                            break;
                        } else {
                            self.is_locked = true;
                        }
                    }
                    Statement::UnLock => {
                        if let Some(p) = self.blocking_queue.pop_front() {
                            self.ready_queue.push_front(p);
                        }
                    }
                    Statement::Yield => {
                        running_process.time_slice = 0;
                    }
                    Statement::End => break,
                }
                if running_process.time_slice == 0 {
                    running_process.time_slice = self.time_slice;
                    self.ready_queue.push_back(running_process);
                    break;
                }
            }
        }
    }

    fn statement_execution_time(&self, statement: &Statement) -> usize {
        match statement {
            Statement::Assignment(_, _) => self.stmt_exec_time_cfg[0],
            Statement::AddAssignment(_, _) => self.stmt_exec_time_cfg[1],
            Statement::Print(_) => self.stmt_exec_time_cfg[2],
            Statement::Lock => self.stmt_exec_time_cfg[3],
            Statement::UnLock => self.stmt_exec_time_cfg[4],
            Statement::Yield => self.stmt_exec_time_cfg[5],
            Statement::End => self.stmt_exec_time_cfg[6],
        }
    }
}
