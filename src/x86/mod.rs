pub mod basic {
  use core::arch::asm;
  use crate::VTable;

  pub const VTABLE: VTable = VTable { switch };

 #[inline(never)]
  pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
    // I hate to be nice about intel things but this one is almost pure
    // elegance in its simplicity of implementation.
    asm!(
      "mov esp, edi", // save the stack pointer to save_to
      "mov esi, esp", // load the stack pointer from resume
      "return",       // continue where we left off
      in("edi") save_to,
      in("esi") resume,
      // note that llvm reserves ebx for unspecified purposes. we hope it doesn't fuck something up.
      out("eax")  _, out("ecx")  _, out("edx")  _,
      // MMX/SSE registers
      out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
      out("xmm4") _, out("xmm5") _, out("xmm6") _, out("xmm7") _,
    );
  }

}
