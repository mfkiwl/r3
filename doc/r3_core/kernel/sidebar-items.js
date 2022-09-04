window.SIDEBAR_ITEMS = {"enum":[["ActivateTaskError","Error type for `Task::activate`."],["AdjustTimeError","Error type for `Kernel::adjust_time`."],["BoostPriorityError","Error type for `Kernel::boost_priority` and `Kernel::unboost_priority`."],["ClearInterruptLineError","Error type for `InterruptLine::clear`."],["CpuLockError","Error type for `Kernel::acquire_cpu_lock` and `Kernel::release_cpu_lock`."],["DrainSemaphoreError","Error type for `Semaphore::drain`."],["EnableInterruptLineError","Error type for `InterruptLine::enable` and `InterruptLine::disable`."],["ExitTaskError","Error type for `Kernel::exit_task`."],["GetCurrentTaskError","Error type for `LocalTask::current`."],["GetEventGroupError","Error type for `EventGroup::get`."],["GetSemaphoreError","Error type for `Semaphore::get`."],["GetTaskPriorityError","Error type for `Task::priority`."],["InterruptTaskError","Error type for `Task::interrupt`."],["LockMutexError","Error type for `Mutex::lock`."],["LockMutexTimeoutError","Error type for `Mutex::lock_timeout`."],["MarkConsistentMutexError","Error type for `Mutex::mark_consistent`."],["ParkError","Error type for `Kernel::park`."],["ParkTimeoutError","Error type for `Kernel::park_timeout`."],["PendInterruptLineError","Error type for `InterruptLine::pend`."],["PollEventGroupError","Error type for `EventGroup::poll`."],["PollSemaphoreError","Error type for `Semaphore::poll_one`."],["QueryInterruptLineError","Error type for `InterruptLine::is_pending`."],["QueryMutexError","Error type for `Mutex::is_locked`."],["ResultCode","All result codes (including the one indicating success) that a kernel function can return."],["SetInterruptLinePriorityError","Error type for `InterruptLine::set_priority` and `InterruptLine::set_priority_unchecked`."],["SetTaskPriorityError","Error type for `Task::set_priority`."],["SetTimerDelayError","Error type for `Timer::set_delay`."],["SetTimerPeriodError","Error type for `Timer::set_period`."],["SignalSemaphoreError","Error type for `Semaphore::signal`."],["SleepError","Error type for `Kernel::sleep`."],["StartTimerError","Error type for `Timer::start`."],["StopTimerError","Error type for `Timer::stop`."],["TimeError","Error type for `Kernel::time` and `Kernel::set_time`."],["TryLockMutexError","Error type for `Mutex::try_lock`."],["UnlockMutexError","Error type for `Mutex::unlock`."],["UnparkError","Error type for `Task::unpark`."],["UnparkExactError","Error type for `Task::unpark_exact`."],["UpdateEventGroupError","Error type for `EventGroup::set` and `EventGroup::clear`."],["WaitError","Error type for wait operations such as `EventGroup::wait`."],["WaitEventGroupError","Error type for `EventGroup::wait`."],["WaitEventGroupTimeoutError","Error type for `EventGroup::wait_timeout`."],["WaitSemaphoreError","Error type for `Semaphore::wait_one`."],["WaitSemaphoreTimeoutError","Error type for `Semaphore::wait_one_timeout`."],["WaitTimeoutError","Error type for wait operations with timeout such as `EventGroup::wait_timeout`."]],"mod":[["cfg","Kernel configuration"],["event_group","Event groups"],["hook","Hooks"],["hunk","Hunks"],["interrupt","Interrupt lines and handlers"],["mutex","Mutexes"],["prelude","The prelude module. This module re-exports `Kernel` and other extension traits with impl-only-use (`use ... as _`, RFC2166)."],["raw","The low-level kernel interface to be implemented by a kernel implementor."],["raw_cfg","The low-level kernel static configuration interface to be implemented by a kernel implementor."],["semaphore","Semaphores"],["task","Tasks"],["timer","Timers"],["traits","Re-exports all traits defined under this module for convenience."]],"trait":[["Kernel","Provides access to the global functionalities of a kernel."]]};