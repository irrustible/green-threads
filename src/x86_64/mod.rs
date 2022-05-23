#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;
#[cfg(all(not(unix), not(windows)))]
compile_error!("Unsupported platform. `green-threads` supports unix or windows");

#[derive(Clone,Copy)]
pub enum Variant {
  /// Perhaps surprisingly, we didn't add any more registers on top of
  /// what came with the original opteron for nearly 2 decades.
  Basic,
  /// AVX512 introduced another 24 registers. It pioneered in Knights
  /// Landing (which you probably couldn't afford) in 2016 and then
  /// Skylake-X in 2017
  Avx512,
}

use crate::VTable;
use raw_cpuid::{CpuId, ExtendedFeatures};

impl Variant {
  pub fn detect() -> Variant {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info().expect("Could not query cpuid.").as_str() {
      "AuthenticAMD" => Variant::Basic,
      "GenuineIntel" => {
        if has_avx512(cpuid.get_extended_features().unwrap()) {
          Variant::Avx512
        } else {
          Variant::Basic
        }
      },
      other => panic!("Unknown CPU vendor: {}", other),
    }
  }
  pub fn vtable(variant: Variant) -> crate::VTable {
    match variant {
      Variant::Basic  => basic::VTABLE,
      Variant::Avx512 => avx512::VTABLE,
    }
  }
}

fn has_avx_512(ef: ExtendedFeatures) -> bool {
  ef.has_avx512_ifma() ||
  ef.has_avx512bw()    ||
  ef.has_avx512cd()    ||
  ef.has_avx512dq()    ||
  ef.has_avx512er()    ||
  ef.has_avx512f()     ||
  ef.has_avx512pf()    ||
  ef.has_avx512vl()    ||
  ef.has_avx512vnni()
}
