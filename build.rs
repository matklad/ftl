use std::{env, path::Path, process::Command};

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  Command::new("clang")
    .args(&[
      "src/thread_local.c",
      "-O3",
      "-c",
      "-flto=thin",
      "-o",
    ])
    .arg(&format!("{}/thread_local.o", out_dir))
    .status()
    .unwrap();
  Command::new("ar")
    .args(&["crus", "libthread_local.a", "thread_local.o"])
    .current_dir(&Path::new(&out_dir))
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static=thread_local");
  println!("cargo:rerun-if-changed=src/thread_local.c");
}
