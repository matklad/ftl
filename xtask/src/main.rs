use std::{env, fs, process::Command};

fn main() {
  run("rustc -vV");

  run("clang --version");

  run("ld.lld --version");

  println!(
        "\n  If you get weird linker errors, make sure that the above versions match.\n\
         Combination that works for me: rustc 1.46, clang 10, lld 10.\n"
    );

  {
    run("cargo build --release -p ftl");
    fs::copy("./target/release/ftl", "./target/ftl_no_lto")
      .unwrap();
  }

  {
    let rustflags = "-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld";
    println!("export RUSTFLAGS={:?}", rustflags);
    env::set_var("RUSTFLAGS", rustflags);
    run("cargo build --release -p ftl");
    fs::copy("./target/release/ftl", "./target/ftl_lto")
      .unwrap();
  }

  {
    run("clang -std=c17 -O3 src/main.c -o ./target/ftl_c");
  }

  run("./target/ftl_c");
  run("./target/ftl_no_lto");
  run("./target/ftl_lto");
}

fn run(cmd: &str) {
  println!("$ {}", cmd);
  let cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
  let status =
    Command::new(cmd[0]).args(&cmd[1..]).status().unwrap();
  println!();
  assert!(status.success())
}
