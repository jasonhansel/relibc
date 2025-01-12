use core::mem;
use syscall;

use super::super::types::*;
use super::super::{Pal, PalSignal};
use super::{e, Sys};
use header::errno::EINVAL;
use header::signal::{sigaction, sigset_t};
use header::sys_time::{itimerval, ITIMER_REAL};
use platform::errno;

impl PalSignal for Sys {
    fn getitimer(which: c_int, out: *mut itimerval) -> c_int {
        let path = match which {
            ITIMER_REAL => "itimer:1",
            _ => unsafe {
                errno = EINVAL;
                return -1;
            },
        };

        let fd = e(syscall::open(path, syscall::O_RDONLY | syscall::O_CLOEXEC));
        if fd == !0 {
            return -1;
        }

        let mut spec = syscall::ITimerSpec::default();
        let count = e(syscall::read(fd, &mut spec));

        let _ = syscall::close(fd);

        if count == !0 {
            return -1;
        }

        unsafe {
            (*out).it_interval.tv_sec = spec.it_interval.tv_sec;
            (*out).it_interval.tv_usec = spec.it_interval.tv_nsec / 1000;
            (*out).it_value.tv_sec = spec.it_value.tv_sec;
            (*out).it_value.tv_usec = spec.it_value.tv_nsec / 1000;
        }

        0
    }

    fn kill(pid: pid_t, sig: c_int) -> c_int {
        e(syscall::kill(pid as usize, sig as usize)) as c_int
    }

    fn killpg(pgrp: pid_t, sig: c_int) -> c_int {
        e(syscall::kill(-(pgrp as isize) as usize, sig as usize)) as c_int
    }

    fn raise(sig: c_int) -> c_int {
        Self::kill(Self::getpid(), sig)
    }

    fn setitimer(which: c_int, new: *const itimerval, old: *mut itimerval) -> c_int {
        let path = match which {
            ITIMER_REAL => "itimer:1",
            _ => unsafe {
                errno = EINVAL;
                return -1;
            },
        };

        let fd = e(syscall::open(path, syscall::O_RDWR | syscall::O_CLOEXEC));
        if fd == !0 {
            return -1;
        }

        let mut spec = syscall::ITimerSpec::default();

        let mut count = e(syscall::read(fd, &mut spec));

        if count != !0 {
            unsafe {
                if !old.is_null() {
                    (*old).it_interval.tv_sec = spec.it_interval.tv_sec;
                    (*old).it_interval.tv_usec = spec.it_interval.tv_nsec / 1000;
                    (*old).it_value.tv_sec = spec.it_value.tv_sec;
                    (*old).it_value.tv_usec = spec.it_value.tv_nsec / 1000;
                }

                spec.it_interval.tv_sec = (*new).it_interval.tv_sec;
                spec.it_interval.tv_nsec = (*new).it_interval.tv_usec * 1000;
                spec.it_value.tv_sec = (*new).it_value.tv_sec;
                spec.it_value.tv_nsec = (*new).it_value.tv_usec * 1000;
            }

            count = e(syscall::write(fd, &spec));
        }

        let _ = syscall::close(fd);

        if count == !0 {
            return -1;
        }

        0
    }

    unsafe fn sigaction(sig: c_int, act: *const sigaction, oact: *mut sigaction) -> c_int {
        let new_opt = if act.is_null() {
            None
        } else {
            let m = (*act).sa_mask;
            Some(syscall::SigAction {
                sa_handler: mem::transmute((*act).sa_handler),
                sa_mask: [m as u64, 0],
                sa_flags: (*act).sa_flags as usize,
            })
        };
        let mut old_opt = if oact.is_null() {
            None
        } else {
            Some(syscall::SigAction::default())
        };
        let ret = e(syscall::sigaction(
            sig as usize,
            new_opt.as_ref(),
            old_opt.as_mut(),
        )) as c_int;
        if let Some(old) = old_opt {
            (*oact).sa_handler = mem::transmute(old.sa_handler);
            let m = old.sa_mask;
            (*oact).sa_mask = m[0] as c_ulong;
            (*oact).sa_flags = old.sa_flags as c_ulong;
        }
        ret
    }

    fn sigprocmask(how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int {
        let new_opt = if set.is_null() {
            None
        } else {
            Some([unsafe { *set as u64 }, 0])
        };
        let mut old_opt = if oset.is_null() { None } else { Some([0, 0]) };
        let ret = e(syscall::sigprocmask(
            how as usize,
            new_opt.as_ref(),
            old_opt.as_mut(),
        )) as c_int;
        if let Some(old) = old_opt {
            unsafe { *oset = old[0] as sigset_t };
        }
        ret
    }
}
