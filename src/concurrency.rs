use arrayvec::ArrayVec;


pub trait SchedulerTask {
    fn run_task(&mut self, miliseconds: u32) -> ();
}

pub struct Scheduler<'a> {
    tasks: ArrayVec<&'a dyn SchedulerTask, 10>,
    time_slice: u32
}

impl<'a> Scheduler<'a> {
    pub fn new(tasks: ArrayVec<&'a dyn SchedulerTask, 10>, time_slice: u32)
    -> Scheduler {
        Scheduler { tasks, time_slice }
    }
}

impl<'a> Scheduler<'a> {
    pub fn run(&self) -> () {
        loop {
            for task in self.tasks.iter() {
                task.run_task(self.time_slice);
            }
        }
    }
}
