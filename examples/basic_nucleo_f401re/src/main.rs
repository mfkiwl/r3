#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(unsafe_block_in_unsafe_fn)] // `unsafe fn` doesn't imply `unsafe {}`
#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]
#![no_main]
#![cfg(target_os = "none")]
use constance::{
    kernel::{StartupHook, Task},
    prelude::*,
    sync::Mutex,
};

// Install a global panic handler that uses RTT
use panic_rtt_target as _;

constance_port_arm_m::use_port!(unsafe struct System);

#[derive(Debug)]
struct Objects {
    task1: Task<System>,
    task2: Task<System>,
    mutex1: Mutex<System, u32>,
}

const COTTAGE: Objects = constance::build!(System, configure_app => Objects);

impl constance_port_arm_m::PortCfg for System {}

constance::configure! {
    const fn configure_app(_: &mut CfgBuilder<System>) -> Objects {
        set!(num_task_priority_levels = 4);

        // Initialize RTT (Real-Time Transfer) with a single up channel and set
        // it as the print channel for the printing macros
        new! { StartupHook<_>, start = |_| {
            rtt_target::rtt_init_print!();
        } };

        let task1 = new! { Task<_>, start = task1_body, priority = 2, active = true };
        let task2 = new! { Task<_>, start = task2_body, priority = 3 };

        let mutex1 = call!(Mutex::new);

        Objects {
            task1,
            task2,
            mutex1,
        }
    }
}

fn task1_body(_: usize) {
    COTTAGE.task2.activate().unwrap();
}

fn task2_body(_: usize) {
    loop {
        rtt_target::rprintln!("time = {:?}", System::time().unwrap());
        System::sleep(constance::time::Duration::from_secs(1)).unwrap();
    }
}
