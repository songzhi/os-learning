use super::os::Os;

pub trait Scheduler {
    fn on_tick(&mut self, os: &mut Os);
}