use std::sync::Arc;

use rand::seq::SliceRandom;

use os_learning::scheduling::job::Job;
use os_learning::scheduling::os::Os;
use os_learning::scheduling::process::{PId, Process};
use os_learning::scheduling::scheduler::fcfs::FirstComeFirstServeScheduler;
use os_learning::scheduling::scheduler::ljf::LongestJobFirstScheduler;
use os_learning::scheduling::scheduler::Scheduler;
use os_learning::scheduling::scheduler::sjf::ShortestJobFirstScheduler;

fn run_jobs(cpu_bound_jobs: usize, io_bound_jobs: usize, jobs_desc: &'static str) -> Vec<Os> {
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
    get_schedulers()
        .into_iter()
        .map(|scheduler| {
            let processes = processes.clone();
            std::thread::spawn(move || {
                let mut os = Os::new(processes.clone(), scheduler, jobs_desc);
                os.run();
                os
            })
        })
        .map(|handle| handle.join().expect("failed to run os"))
        .collect()
}

fn get_schedulers() -> Vec<Box<dyn Scheduler + Send>> {
    vec![
        Box::new(FirstComeFirstServeScheduler::new()),
        Box::new(ShortestJobFirstScheduler::new()),
        Box::new(LongestJobFirstScheduler::new()),
    ]
}

fn print_os_stats(os: &Os, is_detailed: bool) {
    println!("{}", os.desc());
    os.totalled_process_stats().printstd();
    if is_detailed {
        os.raw_process_stats().printstd();
    }
}

fn main() {
    pretty_env_logger::init();
    let mut os_list: Vec<Os> = vec![];
    let cpu_bound_test = std::thread::spawn(move || run_jobs(8, 2, "CPU Bound"));
    let io_bound_test = std::thread::spawn(move || run_jobs(8, 2, "I/O Bound"));
    let average_test = std::thread::spawn(move || run_jobs(8, 2, "Average"));
    os_list.extend(cpu_bound_test.join().expect("cpu bound test failed"));
    os_list.extend(io_bound_test.join().expect("io bound test failed"));
    os_list.extend(average_test.join().expect("average test failed"));
    for os in os_list.iter() {
        println!();
        print_os_stats(os, false);
    }
}
