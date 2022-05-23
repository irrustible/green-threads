//! x86_64/windows is basically the same as x86_64/unix except
//! for different argument registers.

pub mod basic {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

  #[inline(never)]
  pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
    asm!(
      "mov rsp, rcx", // save the stack pointer to save_to
      "mov rdx, rsp", // load the stack pointer from resume
      "return",       // continue where we left off
      in("rcx") save_to,
      in("rdx") resume,
      out("rax")   _, out("rsi")   _, out("rdi")   _,
      out("r8")    _, out("r9")    _, out("r10")   _, out("r11")   _,
      out("r12")   _, out("r13")   _, out("r14")   _, out("r15")   _,
      out("xmm0")  _, out("xmm1")  _, out("xmm2")  _, out("xmm3")  _,
      out("xmm4")  _, out("xmm5")  _, out("xmm6")  _, out("xmm7")  _,
      out("xmm8")  _, out("xmm9")  _, out("xmm10") _, out("xmm11") _,
      out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
    );
    unreachable!()
  }
}

pub mod avx512 {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

  #[inline(never)]
  pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
    asm!(
      "mov rsp, rcx", // save the stack pointer to save_to
      "mov rdx, rsp", // load the stack pointer from resume
      "return",       // continue where we left off
      in("rcx") save_to,
      in("rdx") resume,
      out("rax")   _, out("rsi")   _, out("rdi")   _,
      out("r8")    _, out("r9")    _, out("r10")   _, out("r11")   _,
      out("r12")   _, out("r13")   _, out("r14")   _, out("r15")   _,
      out("xmm0")  _, out("xmm1")  _, out("xmm2")  _, out("xmm3")  _,
      out("xmm4")  _, out("xmm5")  _, out("xmm6")  _, out("xmm7")  _,
      out("xmm8")  _, out("xmm9")  _, out("xmm10") _, out("xmm11") _,
      out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
      out("xmm16") _, out("xmm17") _, out("xmm18") _, out("xmm19") _,
      out("xmm20") _, out("xmm21") _, out("xmm22") _, out("xmm23") _,
      out("xmm24") _, out("xmm25") _, out("xmm26") _, out("xmm27") _,
      out("xmm28") _, out("xmm29") _, out("xmm30") _, out("xmm31") _,
      out("k0")    _, out("k1")    _, out("k2")    _, out("k3")    _,
      out("k4")    _, out("k5")    _, out("k6")    _, out("k7")    _,
    );
    unreachable!()
  }
}
 
