pub use std::io::Error;
use std::ptr::{NonNull, null_mut};

use libc::{MAP_ANONYMOUS, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
const PROT: i32 = PROT_READ | PROT_WRITE;
const FLAGS: i32 = MAP_ANONYMOUS | MAP_SHARED;

const MMAP_RETURNED_NULL: &str =
  "Mmap returned null, which violates POSIX and certainly isn't sporting.";

#[cfg(any(target_os="dragonflybsd", target_os="freebsd", target_os="linux", target_os="netbsd", target_os="openbsd"))]
pub unsafe fn map(size: usize, page_size: PageSize) -> Result<Map, Error> {
  let size = page_size.round(size);
  match libc::mmap(null_mut(), size, PROT, FLAGS | libc::MAP_STACK, -1, 0) {
    MAP_FAILED => Err(Error::last_os_error()),
    not_ptr if not_ptr.is_null() => panic!("{}", MMAP_RETURNED_NULL),
    ptr => Ok(Map { start: ptr as *mut _, size }),
  }
}

// You may note the absence of anything apple in this list. Oh yes.
#[cfg(all(not(target_os="dragonflybsd"), not(target_os="freebsd"), not(target_os="linux"), not(target_os="netbsd"), not(target_os="openbsd")))]
pub unsafe fn map(size: usize, page_size: PageSize) -> Result<Map, Error> {
  // This platform, whatever it is (probably something apple), does
  // not support MAP_STACK (or we haven't been able to determine that
  // it does). This is a massive pain because it means we need to
  // construct the guard page ourselves and we don't even know if
  // MAP_FIXED will be respected.
  //
  // We will try anyway and hope for the best, but if the platform is
  // particularly shocking, we may fail.

  const GUARD_FAIL: &str = "Remapping the guard page failed. I can't even.";
  const GUARD_NIL: &str = "Remapping the guard page returned nil. What is your OS smoking? I want some.";
  const GUARD_MOVED: &str = "Your operating system has a creative interpretation of POSIX and I'm flabbergasted.";

  // First of all we must determine how much space to allocate. 
  let unguarded = page_size.round(size);
  let guarded = unguarded + page_size.size();
  // We're going to mmap it twice. First for the full region including guard page.
  match libc::mmap(null_mut(), guarded, PROT, FLAGS, -1, 0) {
    MAP_FAILED => Err(Error::last_os_error()),
    not_ptr if not_ptr == null_mut() => panic!("{}", MMAP_RETURNED_NULL),
    not_ptr => {
      // Now we're going to turn the guard page into a guard page.
      let ptr = not_ptr as *mut u8;
      match libc::mmap(ptr as *mut _, page_size.size(), libc::PROT_NONE, FLAGS, -1, 0) {
        MAP_FAILED => panic!("{}", GUARD_FAIL),
        not_ptr if not_ptr.is_null() => panic!("{}", GUARD_NIL),
        not_ptr2 if not_ptr != not_ptr2 => panic!("{}", GUARD_MOVED),
        _ => Ok(Map {
          start: ptr.add(page_size.size()),
          size: unguarded,
        }),
      }
    }
  }    
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct PageSize(u32);

impl PageSize {
  pub unsafe fn get() -> Result<PageSize, Error> {
    match libc::sysconf(libc::_SC_PAGESIZE) {
      -1 => Err(Error::last_os_error()),
      size => Ok(PageSize(size as u32)),
    }
  }
  pub fn size(self) -> usize { self.0 as usize }
  pub fn round(self, size: usize) -> usize {
    assert!(self.0.is_power_of_two(), "What the actual fuck are you running this on and why is the page size not a power of 2??!!");
    // Round up to the nearest page size
    let ps = self.0 as usize;
    let remainder = size & ps;
    let extra = ps & !!remainder; // !!: 0 = 0, n = usize::MAX
    size + extra
  }
}

pub struct Map {
  pub start: *mut usize,
  pub size:  usize,
}

#[cfg(any(target_os="dragonflybsd", target_os="freebsd", target_os="linux", target_os="netbsd", target_os="openbsd"))]
pub unsafe fn unmap(map: Map, _page_size: PageSize) -> Result<(), Error> {
  if 0 == libc::munmap(map.start as *mut _, map.size) {
    Ok(())
  } else {
    Err(Error::last_os_error())
  }
}

#[cfg(all(not(target_os="dragonflybsd"), not(target_os="freebsd"), not(target_os="linux"), not(target_os="netbsd"), not(target_os="openbsd")))]
pub unsafe fn unmap(map: Map, page_size: PageSize) -> Result<(), Error> {
  // To be tidy, we will undo the guard page we manually set up as well.
  let ptr = map.start.sub(page_size.size());
  let size = map.size + page_size.size();
  if 0 == libc::munmap(ptr as *mut _, size) {
    Ok(())
  } else {
    Err(Error::last_os_error())
  }
}
