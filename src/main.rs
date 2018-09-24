// #[macro_use]
// extern crate quicli;
// use quicli::prelude::*;

use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("sh")
        .current_dir("../test-nire")
        .arg(get_elm_path())
        .arg("init")
        .arg("--help")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    // println!("status {:?}", output.status);
    // println!("stdout : {}", std::str::from_utf8(&output.stdout).unwrap());
    // println!("stderr : {}", std::str::from_utf8(&output.stderr).unwrap());
}

fn get_elm_path() -> String {
    let output = Command::new("which")
        .arg("elm")
        .output()
        .expect("failed to execute process of get_elm_path");

    std::str::from_utf8(&output.stdout).unwrap().to_string()
}
