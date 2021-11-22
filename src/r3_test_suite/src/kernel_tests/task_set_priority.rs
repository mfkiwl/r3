//! Sequence the execution of tasks by dynamically changing their priorities.
use r3::{
    hunk::Hunk,
    kernel::{traits, Cfg, Task},
};

use super::Driver;
use crate::utils::SeqTracker;

pub trait SupportedSystem:
    traits::KernelBase + traits::KernelTaskSetPriority + traits::KernelStatic
{
}
impl<T: traits::KernelBase + traits::KernelTaskSetPriority + traits::KernelStatic> SupportedSystem
    for T
{
}

pub struct App<System: SupportedSystem> {
    task2: Task<System>,
    seq: Hunk<System, SeqTracker>,
}

impl<System: SupportedSystem> App<System> {
    pub const fn new<C, D: Driver<Self>>(b: &mut Cfg<C>) -> Self
    where
        C: ~const traits::CfgBase<System = System> + ~const traits::CfgTask,
    {
        Task::build()
            .start(task1_body::<System, D>)
            .priority(1)
            .active(true)
            .finish(b);
        let task2 = Task::build()
            .start(task2_body::<System, D>)
            .priority(2)
            .active(true)
            .finish(b);

        let seq = Hunk::<_, SeqTracker>::build().finish(b);

        App { task2, seq }
    }
}

fn task1_body<System: SupportedSystem, D: Driver<App<System>>>(_: usize) {
    // `task1` executes first because it has a higher priority.
    D::app().seq.expect_and_replace(0, 1);
    assert_eq!(D::app().task2.priority(), Ok(2));
    assert_eq!(D::app().task2.effective_priority(), Ok(2));

    // Raise `task2`'s priority to higher than `task1`. `task2` will start
    // executing.
    D::app().task2.set_priority(0).unwrap();

    // Back from `task2`...
    D::app().seq.expect_and_replace(2, 3);
    assert_eq!(D::app().task2.priority(), Ok(2));
    assert_eq!(D::app().task2.effective_priority(), Ok(2));

    // Exit from `task1`, relinquishing the control to `task2`.
}

fn task2_body<System: SupportedSystem, D: Driver<App<System>>>(_: usize) {
    D::app().seq.expect_and_replace(1, 2);
    assert_eq!(D::app().task2.priority(), Ok(0));
    assert_eq!(D::app().task2.effective_priority(), Ok(0));

    // Reset `task2`'s priority. `task1` will resume.
    D::app().task2.set_priority(2).unwrap();

    // `task1` has exited, so `task2` is running again.
    D::app().seq.expect_and_replace(3, 4);

    D::success();
}
