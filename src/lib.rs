#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_as_bytes)]
#![feature(const_vec_new)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
// FIXME: Stable on nightly, remove once redox fork is updated
#![feature(iter_copied)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(thread_local)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::eval_order_dependence)]
#![allow(clippy::mut_from_ref)]

#[macro_use]
extern crate alloc;
extern crate cbitset;
extern crate core_io;
extern crate goblin;
#[macro_use]
extern crate lazy_static;
extern crate memchr;
#[macro_use]
extern crate memoffset;
extern crate posix_regex;
extern crate rand;

#[cfg(target_os = "linux")]
#[macro_use]
extern crate sc;

#[cfg(target_os = "redox")]
extern crate syscall;

#[cfg(target_os = "redox")]
extern crate spin;

#[macro_use]
mod macros;
pub mod c_str;
pub mod cxa;
pub mod db;
pub mod fs;
pub mod header;
pub mod io;
pub mod ld_so;
pub mod mutex;
pub mod platform;
pub mod start;

use platform::{Allocator, Pal, Sys};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn relibc_panic(pi: &::core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let mut w = platform::FileWriter(2);
    let _ = w.write_fmt(format_args!("RELIBC PANIC: {}\n", pi));

    Sys::exit(1);
}

#[cfg(not(test))]
#[panic_handler]
#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(pi: &::core::panic::PanicInfo) -> ! {
    relibc_panic(pi)
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn rust_eh_personality() {}

#[cfg(not(test))]
#[lang = "oom"]
#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn rust_oom(layout: ::core::alloc::Layout) -> ! {
    use core::fmt::Write;

    let mut w = platform::FileWriter(2);
    let _ = w.write_fmt(format_args!(
        "RELIBC OOM: {} bytes aligned to {} bytes\n",
        layout.size(),
        layout.align()
    ));

    Sys::exit(1);
}

#[cfg(not(test))]
#[allow(non_snake_case)]
#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    use core::fmt::Write;

    let mut w = platform::FileWriter(2);
    let _ = w.write_str("_Unwind_Resume\n");

    Sys::exit(1);
}
