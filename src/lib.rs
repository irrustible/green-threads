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

#[cfg(target_arch="aarch64")]
mod aarch64;
#[cfg(target_arch="aarch64")]
pub use aarch64::*;

#[cfg(target_arch="arm")]
mod arm;
#[cfg(target_arch="arm")]
pub use arm::*;

// #[cfg(target_arch="riscv")]
// mod riscv;
// #[cfg(target_arch="riscv")]
// pub use riscv::*;

#[cfg(target_arch="x86")]
mod x86;
#[cfg(target_arch="x86")]
pub use x86::*;

#[cfg(target_arch="x86_64")]
mod x86_64;
#[cfg(target_arch="x86_64")]
pub use x86_64::*;

#[cfg(all(not(target_arch="aarch64"), not(target_arch="arm"), not(target_arch="x86"), not(target_arch="x86_64")))]
compile_error!("Unsupported architecture! We currently support: aarch64, arm, x86, x86_64");


#[derive(Clone,Copy,Eq,PartialEq)]
pub struct VTable {
  pub switch: unsafe extern "C" fn(save_to: *mut usize, resume: *mut usize)
}

pub mod mmap;

