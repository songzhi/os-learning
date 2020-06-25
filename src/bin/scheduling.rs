use std::sync::Arc;

use os_learning::scheduling::{
    FirstComeFirstServeScheduler, HighestResponseRatioNextScheduler, Job, LongestJobFirstScheduler,
    MultilevelFeedbackQueueScheduler, Os, PId, Process, RoundRobinScheduler, Scheduler,
    ShortestJobFirstScheduler, ShortestRemainingJobFirstScheduler,
};

fn run_jobs(cpu_bound_jobs: usize, io_bound_jobs: usize, jobs_desc: &'static str) -> Vec<Os> {
    let mut processes = (0..cpu_bound_jobs)
        .map(|_| Arc::new(Job::cpu_bound(1000)))
        .chain((0..io_bound_jobs).map(|_| Arc::new(Job::io_bound(1000, 4))))
        .enumerate()
        .map(|(id, job)| (id, Process::new(id, job, fastrand::u64(..2000))))
        .collect::<Vec<_>>();
    fastrand::shuffle(processes.as_mut_slice());
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
        Box::new(HighestResponseRatioNextScheduler::new()),
        Box::new(ShortestJobFirstScheduler::new()),
        Box::new(ShortestRemainingJobFirstScheduler::new()),
        Box::new(LongestJobFirstScheduler::new()),
        // Box::new(LongestRemainingJobFirstScheduler::new()),
        Box::new(FirstComeFirstServeScheduler::new()),
        Box::new(RoundRobinScheduler::new(100)),
        Box::new(MultilevelFeedbackQueueScheduler::new([50, 100])),
    ]
}

fn print_os_list_stats(os_list: &[Os], is_detailed: bool) {
    if is_detailed {
        for os in os_list {
            os.stats_table().printstd();
            os.detailed_process_stats_table().printstd();
        }
    } else {
        Os::os_list_stats_table(os_list).printstd();
    }
}

fn main() {
    pretty_env_logger::init();
    let cpu_bound_test = std::thread::spawn(|| run_jobs(8, 2, "CPU Bound"));
    let io_bound_test = std::thread::spawn(|| run_jobs(2, 8, "I/O Bound"));
    let average_test = std::thread::spawn(|| run_jobs(5, 5, "Average"));
    print_os_list_stats(
        cpu_bound_test
            .join()
            .expect("cpu bound test failed")
            .as_slice(),
        false,
    );
    print_os_list_stats(
        io_bound_test
            .join()
            .expect("io bound test failed")
            .as_slice(),
        false,
    );
    print_os_list_stats(
        average_test.join().expect("average test failed").as_slice(),
        false,
    );
}
