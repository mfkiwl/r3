//! Checks the return codes of disallowed system calls made in an interrupt
//! context.
use r3::kernel::{self, traits, Cfg, InterruptHandler, InterruptLine, Task};

use super::Driver;
use crate::utils::conditional::KernelBoostPriorityExt;

pub trait SupportedSystem:
    traits::KernelBase + traits::KernelInterruptLine + KernelBoostPriorityExt
{
}
impl<T: traits::KernelBase + traits::KernelInterruptLine + KernelBoostPriorityExt> SupportedSystem
    for T
{
}

pub struct App<System: SupportedSystem> {
    int: Option<InterruptLine<System>>,
}

impl<System: SupportedSystem> App<System> {
    pub const fn new<C, D: Driver<Self>>(b: &mut Cfg<C>) -> Self
    where
        C: ~const traits::CfgBase<System = System>
            + ~const traits::CfgTask
            + ~const traits::CfgInterruptLine,
    {
        Task::build()
            .start(task_body::<System, D>)
            .priority(0)
            .active(true)
            .finish(b);

        let int = if let (&[int_line, ..], &[int_pri, ..]) =
            (D::INTERRUPT_LINES, D::INTERRUPT_PRIORITIES)
        {
            InterruptHandler::build()
                .line(int_line)
                .start(isr::<System, D>)
                .finish(b);

            Some(
                InterruptLine::build()
                    .line(int_line)
                    .priority(int_pri)
                    .enabled(true)
                    .finish(b),
            )
        } else {
            None
        };

        App { int }
    }
}

fn task_body<System: SupportedSystem, D: Driver<App<System>>>(_: usize) {
    let int = if let Some(int) = D::app().int {
        int
    } else {
        log::warn!("No interrupt lines defined, skipping the test");
        D::success();
        return;
    };

    int.pend().unwrap();
}

fn isr<System: SupportedSystem, D: Driver<App<System>>>(_: usize) {
    // Disallowed in a non-task context
    assert_eq!(
        D::app().int.unwrap().set_priority(1),
        Err(kernel::SetInterruptLinePriorityError::BadContext),
    );
    assert_eq!(
        unsafe { D::app().int.unwrap().set_priority_unchecked(1) },
        Err(kernel::SetInterruptLinePriorityError::BadContext),
    );
    if let Some(cap) = System::BOOST_PRIORITY_CAPABILITY {
        assert_eq!(
            System::boost_priority(cap),
            Err(kernel::BoostPriorityError::BadContext),
        );
    }
    assert_eq!(
        unsafe { System::exit_task() },
        Err(kernel::ExitTaskError::BadContext),
    );

    // Blocking system services
    assert_eq!(System::park(), Err(kernel::ParkError::BadContext));

    D::success();
}
