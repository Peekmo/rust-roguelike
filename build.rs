#![allow(unstable)] 

use std::io::Command;

fn main() {
    Command::new("cp").arg(format!("terminal.png"))
                      .arg(format!("target/")).status().unwrap();
}

