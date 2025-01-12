use super::super::types::*;
use super::super::Pal;
use header::signal::{sigaction, sigset_t};
use header::sys_time::itimerval;

pub trait PalSignal: Pal {
    fn getitimer(which: c_int, out: *mut itimerval) -> c_int;

    fn kill(pid: pid_t, sig: c_int) -> c_int;

    fn killpg(pgrp: pid_t, sig: c_int) -> c_int;

    fn raise(sig: c_int) -> c_int;

    fn setitimer(which: c_int, new: *const itimerval, old: *mut itimerval) -> c_int;

    unsafe fn sigaction(sig: c_int, act: *const sigaction, oact: *mut sigaction) -> c_int;

    fn sigprocmask(how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
}
