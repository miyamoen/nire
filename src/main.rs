// #[macro_use]
// extern crate quicli;
// use quicli::prelude::*;

use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("elm.cmd")
        .current_dir("../test-nire")
        // .arg("init")
        // .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    println!("test");
    println!("{:?}", output.status);
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    println!("aftertest");
}
