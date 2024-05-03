// Import the required dependencies.
use serde_derive::Deserialize;
use std::fs::{self, read_to_string};
use std::process::Command;
use std::process::exit;
//use std::fmt;
use toml;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct Config {
    script: String,
}

fn main() {
    // Variable that holds the filename as a `&str`.
    let filename = "c:\\config\\test.toml";

    // Read the contents of the file using a `match` block 
    // to return the `data: Ok(c)` as a `String` 
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Use a `match` block to return the 
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    let script = data.config.script;
    // Print out the values to `stdout`.
    println!("{}", script);
    //let create_shortcut = include_str!("script.ps1");
    let output = Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process");
    let hello = output.stdout;
    match String::from_utf8(hello) {
        Ok(string) => println!("{}", string),
        Err(e) => println!("Error: {}", e),
    }

}