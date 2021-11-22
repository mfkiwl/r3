//! Checks that when a stack is automatically allocated, both ends of the
//! stack region are aligned to a port-specific alignment.
use core::marker::PhantomData;
use r3::kernel::{traits, Cfg, Task};
use r3_kernel::System;
use r3_test_suite::kernel_tests::Driver;

use r3_port_std::PortInstance;

pub trait SupportedSystemTraits: PortInstance {}
impl<T: PortInstance> SupportedSystemTraits for T {}

pub struct App<System> {
    _phantom: PhantomData<System>,
}

impl<Traits: SupportedSystemTraits> App<System<Traits>> {
    pub const fn new<C, D: Driver<Self>>(b: &mut Cfg<C>) -> Self
    where
        C: ~const traits::CfgBase<System = System<Traits>> + ~const traits::CfgTask,
    {
        Task::build()
            .start(task_body::<Traits, D>)
            .priority(0)
            .active(true)
            .stack_size(4095)
            .finish(b);

        App {
            _phantom: PhantomData,
        }
    }
}

fn task_body<Traits: SupportedSystemTraits, D: Driver<App<System<Traits>>>>(_: usize) {
    let expected_alignment = System::STACK_ALIGN;
    for task_cb in System::task_cb_pool() {
        let stack = task_cb.attr.stack.as_ptr();
        let start = stack as *mut u8;
        let end = start.wrapping_add(stack.len());
        log::trace!("stack = {:?}..{:?}", start, end);

        assert_eq!(start as usize % expected_alignment, 0);
        assert_eq!(end as usize % expected_alignment, 0);
    }
    D::success();
}
