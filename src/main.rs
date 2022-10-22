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


        let args: Vec<String> = Vec::default();
        let args = shlex::split(&line).ok_or("error: Invalid quoting")?;
        //let cli = Cli::parse_from(args);
        let cli = Cli::parse();


        match &cli.command {
            Commands::SET { key, value, command} =>  {
                if let Some(command) = command {
                    match command {
                        SetCommands::EX { seconds } => {
                            // how to keep track of expire time?
                            command_success();
                        }
                        SetCommands::PX { milliseconds } => {

                            command_success();
                        }
                        SetCommands::EXAT { time_seconds } => {

                            command_success();
                        }
                        SetCommands::PXAT { time_milliseconds } => {

                            command_success();
                        }
                        SetCommands::KEEPTTL => {

                            command_success();
                        }
                        SetCommands::NX => {
                            if !hash.contains_key(key) {
                                hash.insert(key.clone(), value.clone() );
                                command_success();
                            }
                        }
                        SetCommands::XX => {
                            if hash.contains_key(key) {
                                hash.insert(key.clone(), value.clone() );
                                command_success();
                            }
                        }
                        SetCommands::GET => {
                            if let Some(value) = hash.get(key) {
                                println!("{}", value)
                            } else {
                                println!("(nil)")
                            }
                        }
                    }
                }
                hash.insert(key.clone(), value.clone());
            }
            Commands::DEL { keys } => {
                println!("{:?}", keys);
            }
            Commands::GET { key } => {
                println!("{:?}", key);
            }
            _ => {}
        }
    }
}

// struct Line(String);

// impl Into<OsString> for Line {
//     fn into(self) -> OsString {
//         OsString::from(self.0)
//     }
// }

// impl IntoIterator for Line {
//     type Item = OsString;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         let s = String::from();
//         let sl = "";
//
//         let os = OsString::from(s);
//
//         self.0.into_iter()
//     }
// }

 fn command_success() {
     println!("OK");
 }

