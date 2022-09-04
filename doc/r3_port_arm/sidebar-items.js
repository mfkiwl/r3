window.SIDEBAR_ITEMS = {"enum":[["InterruptLineTriggerMode","Specifies the type of signal transition that pends an interrupt."],["SetInterruptLineTriggerModeError",""]],"macro":[["use_gic","Implement `PortInterrupts`, `InterruptController`, and [`Gic`] on the given kernel trait type using the General Interrupt Controller (GIC) on the target. Requires [`GicOptions`]."],["use_port","Define a kernel trait type implementing `PortThreading` and [`EntryPoint`]. Requires [`ThreadingOptions`], [`InterruptController`], and [`Timer`]."],["use_sp804","Attach the implementation of `PortTimer` that is based on Arm PrimeCell SP804 Dual Timer to a given kernel trait type. This macro also implements `Timer` on the kernel trait type. Requires [`Sp804Options`]."],["use_startup","Generate startup code. Requires [`StartupOptions`] and `EntryPoint` to be implemented."]],"mod":[["_changelog_","Changelog"]],"struct":[["MemoryMapSection",""]],"trait":[["EntryPoint","Defines the entry points of a port instantiation. Implemented by [`use_port!`]."],["Gic","Provides access to a system-global GIC instance. Implemented by [`use_gic!`]."],["GicOptions","The options for [`use_gic!`]."],["InterruptController","An abstract interface to an interrupt controller. Implemented by [`use_gic!`]."],["Sp804Options","The options for [`use_sp804!`]."],["StartupOptions","The options for [`use_startup!`]."],["ThreadingOptions","The configuration of the port."],["Timer","An abstract inferface to a port timer driver. Implemented by [`use_sp804!`]."]]};