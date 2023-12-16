extern crate clap;
extern crate yaml_rust;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::string::String;
use yaml_rust::YamlLoader;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    yaml: PathBuf,
}

fn read_file(s: &str) -> String {
    let file_contents = fs::read_to_string(s).expect("Should have been able to read the file");
    file_contents
}

fn main() {
    let args = Args::parse();
    let contents = read_file(args.yaml.to_str().expect("--yaml should be a valid path"));
    let yaml_contents = YamlLoader::load_from_str(&contents).unwrap();
    println!("Contents {:?}", yaml_contents[0]);
}
