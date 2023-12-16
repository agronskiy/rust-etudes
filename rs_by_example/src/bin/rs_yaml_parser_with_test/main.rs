extern crate clap;
extern crate yaml_rust;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::string::String;
use yaml_rust::YamlLoader;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}
fn read_file(s: &str) -> String {
    let file_contents = fs::read_to_string(s).expect("Should have been able to read the file");
    file_contents
}

fn main() {
    let contents = read_file("some_file.yaml");
    let yaml_contents = YamlLoader::load_from_str(&contents).unwrap();
    println!("Contents {:?}", yaml_contents[0]);
}
