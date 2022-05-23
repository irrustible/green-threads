use core::arch::asm;
use crate::VTable;

pub const VTABLE: VTable = VTable { switch };

#[inline(never)]
pub unsafe extern "C" fn switch(save_to: *mut usize, resume: *mut usize) {
  // Aarch64 is a bit more complicated than intel because it uses a
  // link register which we need to observe the value of. NEON support
  // comes as standard everywhere.
  asm!(
    "stmfd sp!, lr",  // save the link register to the stack
    "str sp, [r0,0]", // save the current stack pointer to save_to
    "ldr sp, [r1,0]", // load the new stack pointer from resume
    "ldmfd sp!, lr",  // restore the link register from the stack
    "bx lr",          // go back to where we were
    in("w0") save_to,
    in("w1") resume,
    // General purpose registers
    out("w2")   _, out("w3")   _, out("w4")   _, out("w5")   _,
    out("w6")   _, out("w7")   _, out("w8")   _, out("w9")   _,
    out("w10")  _, out("w11")  _, out("w12")  _, out("w13")  _,
    out("w14")  _, out("w15")  _, out("w16")  _, out("w17")  _,
    out("w18")  _, out("w19")  _, out("w20")  _, out("w21")  _,
    out("w22")  _, out("w23")  _, out("w24")  _, out("w25")  _,
    out("w26")  _, out("w27")  _, out("w28")  _, out("w29")  _, /* LR SP */
    // NEON registers
    out("v0")   _, out("v1")   _, out("v2")   _, out("v3")   _,
    out("v4")   _, out("v5")   _, out("v6")   _, out("v7")   _,
    out("v8")   _, out("v9")   _, out("v10")  _, out("v11")  _,
    out("v12")  _, out("v13")  _, out("v14")  _, out("v15")  _,
    out("v16")  _, out("v17")  _, out("v18")  _, out("v19")  _,
    out("v20")  _, out("v21")  _, out("v22")  _, out("v23")  _,
    out("v24")  _, out("v25")  _, out("v26")  _, out("v27")  _,
    out("v28")  _, out("v29")  _, out("v30")  _, out("v31")  _,
  );
  unreachable!()
}
