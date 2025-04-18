#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

fn main() {
    println!("Building workspace ...");
    cli_run::cli_run("cargo", vec!["update"]);
}
