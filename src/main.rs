extern crate core;

mod cli;

use std::collections::{HashMap, LinkedList};
use std::io::{stdin, stdout, Write};
use clap::Parser;
use crate::cli::{Cli, Commands, SetCommands};

fn main() -> Result<(), String> {
    let mut hash: HashMap<String, String> = HashMap::default();
    let mut lists: Vec<LinkedList<String>> = Vec::default();

    loop {
        let mut line = String::new();

        print!(">> ");
        let _ = stdout().flush(); // Flushed the input buffering. Used for formatting.
        stdin().read_line(&mut line).unwrap();
        let line = line.trim();


        let mut args = shlex::split(&line).ok_or("error: Invalid quoting")?;
        args.insert(0, "".to_string());
        //let cli = Cli::parse_from(args);
        let cli = Cli::parse_from(&args);


        match &cli.command {
            Commands::SET { key, value, command} =>  {
                if let Some(command) = command {
                    match command {
                        SetCommands::EX { seconds } => {
                            // how to keep track of expire time?
                        }
                        SetCommands::PX { milliseconds } => {

                        }
                        SetCommands::EXAT { time_seconds } => {

                        }
                        SetCommands::PXAT { time_milliseconds } => {

                        }
                        SetCommands::KEEPTTL => {

                            command_success();
                        }
                        SetCommands::NX => {
                            if !hash.contains_key(key) {
                                hash.insert(key.clone(), value.clone() );
                            }
                        }
                        SetCommands::XX => {
                            if hash.contains_key(key) {
                                hash.insert(key.clone(), value.clone() );
                            }
                        }
                        SetCommands::GET => {
                            if let Some(value) = hash.get(key) {
                                found_value(value)
                            } else {
                                found_nil()
                            }
                        }
                    }
                }
                hash.insert(key.clone(), value.clone());
                command_success();
            }
            Commands::DEL { keys } => {
                let mut keys_deleted = 0;
                for key in keys {
                    if hash.contains_key(key) {
                        hash.remove(key);
                        keys_deleted += 1;
                    }
                }
                println!("(ingeger) {}", keys_deleted);
            }
            Commands::GET { key } => {
                if let Some(value) = hash.get(key) {
                    found_value(value)
                } else {
                    found_nil()
                }
            }
            _ => {}
        }
    }
}

fn found_value(value: &String) {
    println!("{}", value);
}

fn command_success() {
    println!("OK");
}

fn found_nil() {
   println!("(nil)")
}

