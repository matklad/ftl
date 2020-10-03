#![cfg_attr(feature = "unstable", feature(thread_local))]

use std::{cell::Cell, time::Instant};

const STEPS: u32 = 1_000_000_000;
fn sum_rust() -> u32 {
  thread_local! {
    static COUNTER: Cell<u32> = Cell::new(0);
  }

  for step in 0..STEPS {
    COUNTER.with(|it| {
      let inc = step.wrapping_mul(step) ^ step;
      it.set(it.get().wrapping_add(inc))
    })
  }
  COUNTER.with(|it| it.get())
}

fn with_c_counter<T>(f: impl FnOnce(&Cell<u32>) -> T) -> T {
  extern "C" {
    fn get_thread_local() -> *mut u32;
  }
  let counter =
    unsafe { &*(get_thread_local() as *mut Cell<u32>) };
  f(&counter)
}

fn sum_rust_c() -> u32 {
  for step in 0..STEPS {
    with_c_counter(|it| {
      let inc = step.wrapping_mul(step) ^ step;
      it.set(it.get().wrapping_add(inc))
    })
  }
  with_c_counter(|it| it.get())
}

static mut KEY: libc::pthread_key_t = 0;

fn with_pthread_counter<T>(
  f: impl FnOnce(&Cell<u32>) -> T,
) -> T {
  unsafe {
    let ptr = libc::pthread_getspecific(KEY);
    let cell: &Cell<u32> = &*ptr.cast();
    f(cell)
  }
}

fn sum_pthread() -> u32 {
  for step in 0..STEPS {
    with_pthread_counter(|it| {
      let inc = step.wrapping_mul(step) ^ step;
      it.set(it.get().wrapping_add(inc))
    })
  }
  with_pthread_counter(|it| it.get())
}

fn sum_local() -> u32 {
  let mut counter = 0u32;
  for step in 0..STEPS {
    let inc = step.wrapping_mul(step) ^ step;
    counter = counter.wrapping_add(inc)
  }
  counter
}

#[cfg(feature = "unstable")]
fn sum_rust_unstable() -> u32 {
  #[thread_local]
  static COUNTER: Cell<u32> = Cell::new(0);

  for step in 0..STEPS {
    let inc = step.wrapping_mul(step) ^ step;
    COUNTER.set(COUNTER.get().wrapping_add(inc))
  }
  COUNTER.get()
}
fn main() {
  {
    let t = Instant::now();
    let r = sum_rust();
    eprintln!(
      "Rust:            {} {}ms",
      r,
      t.elapsed().as_millis()
    );
  }
  {
    let t = Instant::now();
    let r = sum_rust_c();
    eprintln!(
      "Rust/C:          {} {}ms",
      r,
      t.elapsed().as_millis()
    );
  }
  #[cfg(feature = "unstable")]
  {
    let t = Instant::now();
    let r = sum_rust_unstable();
    eprintln!(
      "#[thread_local]: {} {}ms",
      r,
      t.elapsed().as_millis()
    );
  }
  {
    unsafe {
      let cell: Box<Cell<u32>> = Box::new(Cell::new(0u32));
      let cell = Box::into_raw(cell);
      unsafe extern "C" fn free(ptr: *mut libc::c_void) {
        let _: Box<Cell<u32>> = Box::from_raw(ptr.cast());
      }
      libc::pthread_key_create(&mut KEY as *mut _, Some(free));
      libc::pthread_setspecific(KEY, cell.cast());
    }
    let t = Instant::now();
    let r = sum_pthread();
    eprintln!(
      "pthread:         {} {}ms",
      r,
      t.elapsed().as_millis()
    );
  }
  {
    let t = Instant::now();
    let r = sum_local();
    eprintln!(
      "local variable:  {} {}ms",
      r,
      t.elapsed().as_millis()
    );
  }
}
