use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?; // read an exisiting file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?; // used to create a new file
    file.write_all(b"Hello, world!")?; // used to write the Hello, World in the file
    Ok(())
}

use std::{fs, io};

fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let prefix = match file_type {
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        };
        println!("{prefix} {}", entry.path().display());
    }

    Ok(())
}