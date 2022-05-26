use std::ptr::null_mut;

use green_threads::*;

fn counter(counter: *mut u8, me: *mut *mut usize, them: *mut *mut usize) {
  unsafe {
    counter.write(counter.read() + 1);
    resume(me, them);
    counter.write(counter.read() + 1);
    resume(me, them);
    counter.write(counter.read() + 1);
    resume(me, them);
    counter.write(counter.read() + 1);
    resume(me, them);
    counter.write(counter.read() + 1);
    resume(me, them);
  }
}

#[test]
fn test_friend() {
  let page_size = unsafe { alloc::PageSize::get() }.unwrap();
  let map = unsafe { alloc::map(4096, page_size) }.unwrap();
  let mut count: u8 = 0;
  let mut me:   *mut usize = null_mut();
  let mut them: *mut usize = map.start ;
  assert_eq!(count, 0);
  unsafe { spawn(&mut me, &mut them, || counter(&mut count, &mut them, &mut me)); }
  // assert_eq!(count, 1);
  // unsafe { resume(&mut me, &mut them) }
  // assert_eq!(count, 2);
  // unsafe { resume(&mut me, &mut them) }
  // assert_eq!(count, 3);
  // unsafe { resume(&mut me, &mut them) }
  // assert_eq!(count, 4);
  // unsafe { resume(&mut me, &mut them) }
  // assert_eq!(count, 5);
}

// #[test]
// fn ping_pong() {
// }
