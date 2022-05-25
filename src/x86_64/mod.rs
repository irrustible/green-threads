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

pub mod mmap;

#[derive(Clone,Copy)]
pub enum Profile {
  /// Perhaps surprisingly, we didn't add any more registers on top of
  /// what came with the original opteron for nearly 2 decades.
  Basic,
  /// AVX512 introduced another 24 registers. It pioneered in Knights
  /// Landing (which you probably couldn't afford) in 2016 and then
  /// Skylake-X in 2017
  Avx512,
}

use raw_cpuid::{CpuId, ExtendedFeatures};

impl Profile {
  pub fn detect() -> Profile {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info().expect("Could not query cpuid.").as_str() {
      "AuthenticAMD" => Profile::Basic,
      "GenuineIntel" => {
        if has_avx512(CpuId::get_extended_feature_info(&cpuid).unwrap()) {
          Profile::Avx512
        } else {
          Profile::Basic
        }
      },
      other => panic!("Unknown CPU vendor: {}", other),
    }
  }
  pub fn vtable(variant: Profile) -> crate::VTable {
    match variant {
      Profile::Basic  => basic::VTABLE,
      Profile::Avx512 => avx512::VTABLE,
    }
  }
}

use mmap_rs::{MmapFlags, MmapOptions};
pub use mmap_rs::error::Error;

/// Allocate memory for an autogrow stack with a given maximum size.
///
/// Note: Unlikely to succeed if the size is not a multiple of `MmapInfo::get().page_size`
pub fn allocate_stack(size: usize) -> Result<MmapMut, Error> {
  Ok(MmapOptions::new(size)
     .with_flags(MmapFlags::STACK)
     .map_mut()?
     .as_mut_ptr()
     .add(size))
}

fn has_avx512(ef: ExtendedFeatures) -> bool {
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
