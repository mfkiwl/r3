use core::{marker::PhantomData, num::NonZeroUsize};

use crate::{
    kernel::{
        cfg::{cfg_new_hunk_zero_array, CfgBuilder},
        task,
        utils::CpuLockCell,
        Port,
    },
    utils::Init,
};

/// Configuration builder type for [`Task`].
///
/// [`Task`]: crate::kernel::Task
#[doc(hidden)]
pub struct CfgTaskBuilder<System> {
    _phantom: PhantomData<System>,
    start: Option<fn(usize)>,
    param: usize,
    stack: Option<TaskStack<System>>,
    priority: Option<usize>,
    active: bool,
}

enum TaskStack<System> {
    Auto(usize),
    Hunk(task::StackHunk<System>),
    // TODO: Externally supplied stack? It's blocked by
    //       <https://github.com/rust-lang/const-eval/issues/11>, I think
}

impl<System: Port> CfgTaskBuilder<System> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
            start: None,
            param: 0,
            stack: None,
            priority: None,
            active: false,
        }
    }

    pub const fn start(self, start: fn(usize)) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    pub const fn param(self, param: usize) -> Self {
        Self { param, ..self }
    }

    pub const fn stack_size(self, stack_size: usize) -> Self {
        // FIXME: `Option::is_some` is not `const fn` yet
        if let Some(_) = self.stack {
            panic!("the task's stack is already specified");
        }

        Self {
            stack: Some(TaskStack::Auto(stack_size)),
            ..self
        }
    }

    pub const fn stack_hunk(self, stack_hunk: task::StackHunk<System>) -> Self {
        // FIXME: `Option::is_some` is not `const fn` yet
        if let Some(_) = self.stack {
            panic!("the task's stack is already specified");
        }

        Self {
            stack: Some(TaskStack::Hunk(stack_hunk)),
            ..self
        }
    }

    pub const fn priority(self, priority: usize) -> Self {
        Self {
            priority: Some(priority),
            ..self
        }
    }

    pub const fn active(self, active: bool) -> Self {
        Self { active, ..self }
    }

    pub const fn finish(self, cfg: &mut CfgBuilder<System>) -> task::Task<System> {
        // FIXME: `Option::unwrap_or` is not `const fn` yet
        let stack = if let Some(stack) = self.stack {
            stack
        } else {
            TaskStack::Auto(System::STACK_DEFAULT_SIZE)
        };
        let stack = match stack {
            TaskStack::Auto(size) => {
                let hunk = cfg_new_hunk_zero_array(cfg, size, System::STACK_ALIGN);

                // Safety: We just created a hunk just for this task, and we
                // don't use this hunk for other purposes.
                unsafe { task::StackHunk::from_hunk(hunk) }
            }
            TaskStack::Hunk(hunk) => hunk,
        };

        let inner = &mut cfg.inner;

        inner.tasks.push(CfgBuilderTask {
            start: if let Some(x) = self.start {
                x
            } else {
                panic!("`start` (task entry point) is not specified")
            },
            param: self.param,
            stack,
            priority: if let Some(x) = self.priority {
                x
            } else {
                panic!("`priority` is not specified")
            },
            active: self.active,
        });

        unsafe { task::Task::from_id(NonZeroUsize::new_unchecked(inner.tasks.len())) }
    }
}

#[doc(hidden)]
pub struct CfgBuilderTask<System> {
    start: fn(usize),
    param: usize,
    stack: task::StackHunk<System>,
    priority: usize,
    active: bool,
}

impl<System> Clone for CfgBuilderTask<System> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            param: self.param,
            stack: self.stack,
            priority: self.priority,
            active: self.active,
        }
    }
}

impl<System> Copy for CfgBuilderTask<System> {}

impl<System: Port> CfgBuilderTask<System> {
    pub const fn to_state(&self, attr: &'static task::TaskAttr<System>) -> task::TaskCb<System> {
        task::TaskCb {
            port_task_state: System::PORT_TASK_STATE_INIT,
            attr,
            priority: if self.priority < System::NUM_TASK_PRIORITY_LEVELS {
                System::TASK_PRIORITY_LEVELS[self.priority]
            } else {
                panic!("task's `priority` must be less than `num_task_priority_levels`");
            },
            st: CpuLockCell::new(if self.active {
                task::TaskSt::PendingActivation
            } else {
                task::TaskSt::Dormant
            }),
            link: CpuLockCell::new(None),
            wait: Init::INIT,
            _force_int_mut: crate::utils::RawCell::new(()),
        }
    }

    pub const fn to_attr(&self) -> task::TaskAttr<System> {
        task::TaskAttr {
            entry_point: self.start,
            entry_param: self.param,
            stack: self.stack,
        }
    }
}
