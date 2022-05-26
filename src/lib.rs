//! Raw userland context-switching primitives.
//!
//! Set fire to code before using. May even work.
//!
//! The basic strategy is to mark most registers as clobbered so that
//! the compiler will spill them to the stack as required.
//!
//! The switch thus is basically three or five operations:
//! * Save the link register (ARM/AArch64 only).
//! * Save the stack pointer to the 'current' pointer.
//! * Load the stack pointer from the 'resume' pointer.
//! * Restore the link register (ARM/AArch64 only).
//! * Return from function call.

#[cfg(all(not(target_arch="aarch64"), not(target_arch="arm"), not(target_arch="x86"), not(target_arch="x86_64")))]
compile_error!("Unsupported architecture! `green-threads` currently supports aarch64, arm, x86 and x86_64");

#[cfg(all(not(unix), not(windows)))]
compile_error!("Unsupported platform. `green-threads` currently supports unix and windows");

pub mod alloc;

use core::arch::asm;

/// Pauses the current green thread's execution to save_to and spawns the provided closure on the
///
/// # Safety
///
/// The provided function to run on the new stack must not return. It should as its last act perform
/// whatever cleanup is required (e.g. removing from scheduler, freeing memory) and then continue
/// another task. If it is the last task, it should probably exit the program.

pub unsafe extern "C" fn spawn<F>(save_to: *mut *mut usize, new_stack: *mut *mut usize, f: F)
where F: FnOnce() {
  #[cfg(target_arch="aarch64")]
  asm!(
    "stmfd rsp!, lr",          // save the link register to the stack
    "str rsp, [{save_to}, 0]", // save the current stack pointer to save_to
    "ldr rsp, [{resume},  0]", // load the new stack pointer from resume
    save_to   = in(reg) save_to,
    new_stack = in(reg) new_stack,
    clobber_abi("C"),
  );
  #[cfg(target_arch="arm")]
  asm!(
    "stmfd sp!, lr",            // save the link register to the stack
    "str sp, [{save_to},   0]", // save the current stack pointer to save_to
    "ldr sp, [{new_stack}, 0]", // load the new stack pointer from new_stack
    save_to   = in(reg) save_to,
    new_stack = in(reg) new_stack,
    clobber_abi("C")
  );
  #[cfg(target_arch="x86")]
  asm!(
    "mov esp, {save_to}",   // save the stack pointer to save_to
    "mov {new_stack}, esp", // load the stack pointer from new_stack
    save_to   = in(reg) save_to,
    new_stack = in(reg) new_stack,
    clobber_abi("C")
  );
  #[cfg(target_arch="x86_64")]
  asm!(
    "mov rsp, {save_to}",   // save the stack pointer to save_to
    "mov {new_stack}, rsp", // load the stack pointer from new_stack
    save_to   = in(reg) save_to,
    new_stack = in(reg) new_stack,
    clobber_abi("C")
  );
  f()
}

/// Pauses the current green thread's execution to save_to and resumes the provided green thread.
///
/// # Safety
pub unsafe extern "C" fn resume(save_to: *mut *mut usize, resume: *mut *mut usize) {
  #[cfg(target_arch="aarch64")]
  asm!(
    "stmfd rsp!, lr",          // save the link register to the stack
    "str rsp, [{save_to}, 0]", // save the current stack pointer to save_to
    "ldr rsp, [{resume},  0]", // load the new stack pointer from resume
    "ldmfd rsp!, lr",          // restore the link register from the stack
    "bx lr",                   // go back to where we were
    save_to = in(reg) save_to,
    resume  = in(reg) resume,
    clobber_abi("C")
  );
  #[cfg(target_arch="arm")]
  asm!(
    "stmfd sp!, lr",          // save the link register to the stack
    "str sp, [{save_to}, 0]", // save the current stack pointer to save_to
    "ldr sp, [{resume},  0]", // load the new stack pointer from resume
    "ldmfd sp!, lr",          // restore the link register from the stack
    "bx lr",                  // go back to where we were
    save_to = in(reg) save_to,
    resume  = in(reg) resume,
    clobber_abi("C")
  );
  #[cfg(target_arch="x86")]
  asm!(
    "mov esp, {save_to}", // save the stack pointer to save_to
    "mov {resume}, esp",  // load the stack pointer from resume
    "return",             // continue where we left off
    save_to = in(reg) save_to,
    resume  = in(reg) resume,
    clobber_abi("C")
  );
  #[cfg(target_arch="x86_64")]
  asm!(
    "mov rsp, {save_to}", // save the stack pointer to save_to
    "mov {resume}, rsp",  // load the stack pointer from resume
    "ret",                // continue where we left off
    save_to = in(reg) save_to,
    resume  = in(reg) resume,
    clobber_abi("C")
  );
}
