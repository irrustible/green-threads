pub mod basic {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

  #[inline(never)]
  pub unsafe extern "C" fn save(to: *mut usize) {
    asm!(
      "mov rsp, rdi", // save the stack pointer to save_to
      in("rdi") to,
      // General purpose registers
      out("rax")   _, out("rcx")   _, out("rdx")   _, our("rsi")   _,
      // note that llvm reserves rbx for unspecified purposes. we hope it doesn't fuck something up.
      out("r8")    _, out("r9")    _, out("r10")   _, out("r11")   _,
      out("r12")   _, out("r13")   _, out("r14")   _, out("r15")   _,
      // SSE/AVX/etc registers. Note that ymm and zmm registers overlap with xmm registers.
      out("xmm0")  _, out("xmm1")  _, out("xmm2")  _, out("xmm3")  _,
      out("xmm4")  _, out("xmm5")  _, out("xmm6")  _, out("xmm7")  _,
      out("xmm8")  _, out("xmm9")  _, out("xmm10") _, out("xmm11") _,
      out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
    );
    unreachable!()
  }

  #[inline(never)]
  pub unsafe extern "C" fn switch(pause: *mut usize, resume: *mut usize) {
    // Like x86, the actual assembler here is more or less a lesson in
    // simplicity, but there are many more registers to clobber.
    asm!(
      "mov rsp, rdi", // save the stack pointer to pause
      "mov rsi, rsp", // load the stack pointer from resume
      "return",       // continue where we left off
      in("rdi") pause,
      in("rsi") resume,
      // General purpose registers
      out("rax")   _, out("rcx")   _, out("rdx")   _,
      // note that llvm reserves rbx for unspecified purposes. we hope it doesn't fuck something up.
      out("r8")    _, out("r9")    _, out("r10")   _, out("r11")   _,
      out("r12")   _, out("r13")   _, out("r14")   _, out("r15")   _,
      // SSE/AVX/etc registers. Note that ymm and zmm registers overlap with xmm registers.
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
  pub unsafe extern "C" fn switch(pause: *mut usize, resume: *mut usize) {
    asm!(
      "mov rsp, rdi", // save the stack pointer to pause
      "mov rsi, rsp", // load the stack pointer from resume
      "return",       // continue where we left off
      in("rdi") pause,
      in("rsi") resume,
      out("rax")   _, out("rcx")   _, out("rdx")   _,
      out("r8")    _, out("r9")    _, out("r10")   _, out("r11")   _,
      out("r12")   _, out("r13")   _, out("r14")   _, out("r15")   _,
      out("xmm0")  _, out("xmm1")  _, out("xmm2")  _, out("xmm3")  _,
      out("xmm4")  _, out("xmm5")  _, out("xmm6")  _, out("xmm7")  _,
      out("xmm8")  _, out("xmm9")  _, out("xmm10") _, out("xmm11") _,
      out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
      // AVX512 provides these additional registers for SSE and AVX usage
      out("xmm16") _, out("xmm17") _, out("xmm18") _, out("xmm19") _,
      out("xmm20") _, out("xmm21") _, out("xmm22") _, out("xmm23") _,
      out("xmm24") _, out("xmm25") _, out("xmm26") _, out("xmm27") _,
      out("xmm28") _, out("xmm29") _, out("xmm30") _, out("xmm31") _,
      // AVX512 also provides these mask registers
      out("k0")    _, out("k1")    _, out("k2")    _, out("k3")    _,
      out("k4")    _, out("k5")    _, out("k6")    _, out("k7")    _,
    );
    unreachable!()
  }
}
