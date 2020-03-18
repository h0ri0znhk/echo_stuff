// This program takes in a folder or directory and echos out it's contents with style

// File or directory?
// If file
// print out file as designed
// else
// recursively get a list of all the files inside a directory
// for each file
// print out file as designed

// TODO: read keyboard q to quit instead of ctrl c

use glob::glob;
use rand::distributions::{Distribution, Uniform};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{thread, time};
use structopt::StructOpt;

const PAUSE: [u16; 5] = [15, 25, 35, 35, 100];

#[derive(StructOpt, Debug)]
struct Input {
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() {
    let Input { paths } = Input::from_args();

    for path in paths {
        if path.is_file() {
            // if input.is_file() {

            process_file(&path);
        } else if path.is_dir() {
            println!("{} is a directory", path.display());

            let search = [path.to_str().unwrap(), "**/*.rs"].concat();

            for entry in glob(&search).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => println!("{:?}", process_file(&path)),
                    Err(e) => println!("{:?}", e),
                }
            }
        } else {
            println!("Can't determin what {} is", path.display());
        }
    }
}

fn process_file(input: &Path) {
    println!("Processing {}", input.display());

    let contents = fs::read_to_string(input).expect("Failed to load in file...");

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..5);

    for var in contents.chars() {
        let throw = die.sample(&mut rng);

        let millis = time::Duration::from_millis(PAUSE[throw].into());

        thread::sleep(millis);
        print!("{}", var);
        io::stdout().flush().unwrap();
    }
}
