use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use rand::seq::SliceRandom;

use os_learning::scheduling::job::Job;
use os_learning::scheduling::os::Os;
use os_learning::scheduling::process::{PId, Process};
use os_learning::scheduling::scheduler::fcfs::FCFSScheduler;
use os_learning::scheduling::scheduler::Scheduler;

fn test_jobs(cpu_bound_jobs: usize, io_bound_jobs: usize, schedulers: Vec<Rc<RefCell<dyn Scheduler>>>, is_detailed: bool) {
    let mut processes = (0..cpu_bound_jobs)
        .map(|_| Arc::new(Job::cpu_bound(1000)))
        .chain((0..io_bound_jobs).map(|_| Arc::new(Job::io_bound(1000, 4))))
        .enumerate()
        .map(|(id, job)| (id, Process::new(id, job, rand::random::<u8>() as u64 * 10)))
        .collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    processes.as_mut_slice().shuffle(&mut rng);
    let processes = processes
        .into_iter()
        .collect::<indexmap::IndexMap<PId, Process>>();
    for scheduler in schedulers {
        let mut os = Os::new(processes.clone(), scheduler);
        os.run();
        println!("{}", os.desc());
        os.totalled_process_stats().printstd();
        if is_detailed {
            os.raw_process_stats().printstd();
        }
    }
}

fn main() {
    pretty_env_logger::init();
    let schedulers: Vec<Rc<RefCell<dyn Scheduler>>> = vec![Rc::new(RefCell::new(FCFSScheduler::new()))];
    test_jobs(2, 2, schedulers, false);
}
