#[cfg(all(not(target_os="android"), not(target_os="freebsd"), not(target_os="linux"), not(target_os="openbsd"), not(windows)))]
compile_error!("Unsupported target OS. Currently supported for AArch32: Android, FreeBSD, Linux, OpenBSD, Windows");

#[cfg(not(windows))] // Windows requires NEON
mod basic {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

  #[inline(never)]
  pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
    // 32-bit ARM is kinda a shitty ISA, but for our purposes, it's much
    // like aarch64, just with a funky register layout in the models
    // which have more registers.
    asm!(
      "stmfd sp!, lr",  // save the link register to the stack
      "str sp, [r0,0]", // save the current stack pointer to save_to
      "ldr sp, [r1,0]", // load the new stack pointer from resume
      "ldmfd sp!, lr",  // restore the link register from the stack
      "bx lr",          // go back to where we were
      in("r0") save_to,
      in("r1") resume,
      // General purpose registers
      out("r2")   _, out("r3")   _, out("r4")   _, out("r5")   _,
      out("r6")   _, out("r7")   _, out("r8")   _, out("r9")   _,
      out("r10")  _, out("r11")  _, out("r12")  _, /* SP LR PC */
      out("r16")  _, out("r17")  _, out("r18")  _, out("r19")  _,
      out("w20")  _, out("w21")  _, out("w22")  _, out("w23")  _,
      out("w24")  _, out("w25")  _, out("w26")  _, out("w27")  _,
      out("w28")  _, out("w29")  _, out("w30")  _, out("w31")  _, 
    );
    unreachable!()
  }
}

#[cfg(not(target_os="openbsd"))] // OpenBSD disables NEON on 32-bit ARM.
mod neon {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

  #[inline(never)]
  pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
    // 32-bit ARM is kinda a shitty ISA, but for our purposes, it's much
    // like aarch64, just with a funky register layout in the models
    // which have more registers.
    asm!(
      "stmfd sp!, lr",  // save the link register to the stack
      "str sp, [r0,0]", // save the current stack pointer to save_to
      "ldr sp, [r1,0]", // load the new stack pointer from resume
      "ldmfd sp!, lr",  // restore the link register from the stack
      "bx lr",          // go back to where we were
      in("r0") save_to,
      in("r1") resume,
      // General purpose registers
      out("r2")   _, out("r3")   _, out("r4")   _, out("r5")   _,
      out("r6")   _, out("r7")   _, out("r8")   _, out("r9")   _,
      out("r10")  _, out("r11")  _, out("r12")  _, /* SP LR PC */
      out("r16")  _, out("r17")  _, out("r18")  _, out("r19")  _,
      out("w20")  _, out("w21")  _, out("w22")  _, out("w23")  _,
      out("w24")  _, out("w25")  _, out("w26")  _, out("w27")  _,
      out("w28")  _, out("w29")  _, out("w30")  _, out("w31")  _, 
      // NEON registers
      out("d0")   _, out("d1")   _, out("d2")   _, out("d3")   _,
      out("d4")   _, out("d5")   _, out("d6")   _, out("d7")   _,
      out("d8")   _, out("d9")   _, out("d10")  _, out("d11")  _,
      out("d12")  _, out("d13")  _, out("d14")  _, out("d15")  _,
      out("d16")  _, out("d17")  _, out("d18")  _, out("d19")  _,
      out("d20")  _, out("d21")  _, out("d22")  _, out("d23")  _,
      out("d24")  _, out("d25")  _, out("d26")  _, out("d27")  _,
      out("d28")  _, out("d29")  _, out("d30")  _, out("d31")  _,
    );
    unreachable!()
  }
}

#[derive(Clone,Copy)]
pub enum Variant {
  #[cfg(not(windows))]
  Basic,
  #[cfg(not(target_os = "openbsd"))]
  Neon,
}

use libc::c_ulong;
#[cfg(target_os = "freebsd")]
use libc::{c_int, c_void};

#[cfg(not(windows))]
const HWCAP_NEON: c_ulong = 1 << 12;

impl Variant {
  #[cfg(any(target_os="android", target_os="linux"))]
  pub fn detect() -> Variant {
    use libc::c_ulong;
    extern "C" {
      fn getauxval(type_: c_ulong) -> c_ulong;
    }
    const AT_HWCAP: c_ulong = 16;
    if unsafe { getauxval(AT_HWCAP) & HWCAP_NEON } == HWCAP_NEON {
      Variant::Neon
    } else {
      Variant::Basic
    }
  }

  // #[cfg(target_os="dragonflybsd")]
  // pub fn detect() -> Variant { Variant::Basic }

  #[cfg(target_os="freebsd")]
  pub fn detect() -> Variant {
    use libc::{c_int, c_ulong, c_void};
    extern "C" {
      fn elf_aux_info(aux: c_int, buf: *mut c_void, buflen: c_int) -> c_int;
    }
    const AT_HWCAP: c_int = 25;
    let caps: c_ulong = 0;
    let buffer : *mut c_void = { let caps: *const c_ulong = &caps; caps } as *mut c_void;
    unsafe {
      let _ret = elf_aux_info(AT_HWCAP, buffer, std::mem::size_of_val(&caps) as i32);
    }
    if caps & HWCAP_NEON == HWCAP_NEON {
      Variant::Neon
    } else {
      Variant::Basic
    }
  }

  // #[cfg(target_os="netbsd")]
  // pub fn detect() -> Variant { Variant::Basic }

  #[cfg(target_os="openbsd")] // OpenBSD disables NEON on 32-bit ARM.
  pub fn detect() -> Variant { Variant::Basic }

  #[cfg(windows)] // Windows only supports ARMv7 + NEON
  pub fn detect() -> Variant { Variant::Neon }

  pub fn vtable(self) -> VTable {
    match self {
      #[cfg(not(windows))]
      Self::Basic => basic::VTABLE,
      #[cfg(not(target_os="openbsd"))]
      Self::Neon  => neon::VTABLE,
    }
  }
}
