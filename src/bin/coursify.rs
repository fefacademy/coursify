use std::{path::Path, process::Command};

use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct CoursifyConfig {
    quality: String,
    extension: String,
}

fn main() {
    let matches = App::new("coursify")
        .arg(
            Arg::new("dir")
                .help("The path to the course")
                .takes_value(true)
                .required(true),
        )
        .version("0.1.0")
        .about("A CLI to coursify your content")
        .author("Victor Ndaba")
        .get_matches();

    let mut dir = matches.value_of("dir").unwrap();
    let current = std::env::current_dir().unwrap();

    if dir == "." {
        dir = current.to_str().unwrap();
    }

    coursify(dir);
}

fn coursify(dir: &str) {
    // TODO: Start coursifying, conside MT
    println!(" Coursifying the content at: {}", dir);
    let mut child = Command::new("node");
    let path = Path::new("./").join("site/ffi.js");

    let mut c = child
        .arg(path.as_os_str())
        .arg(dir)
        .spawn()
        .expect("Failed to start the node process");

    c.wait().expect("Failed to wait for child");
}
