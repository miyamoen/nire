// #[macro_use]
// extern crate quicli;
// use quicli::prelude::*;
extern crate notify;

use std::process::{Command, Stdio};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch() -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch("../test-nire/src", RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                elm_make()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}

fn elm_make() {
    Command::new("sh")
        .current_dir("../test-nire")
        .arg(get_elm_path())
        .arg("make")
        .arg("src/Main.elm")
        .arg("--debug")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to execute elm make");
}

fn get_elm_path() -> String {
    let output = Command::new("which")
        .arg("elm")
        .output()
        .expect("failed to execute which elm");

    std::str::from_utf8(&output.stdout).unwrap().to_string()
}
