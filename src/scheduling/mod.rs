pub use job::Job;
pub use os::Os;
pub use process::{PId, Process};
pub use scheduler::*;

pub mod job;
pub mod os;
pub mod process;
pub mod scheduler;
pub mod statement;

const TICK: u64 = 1;
