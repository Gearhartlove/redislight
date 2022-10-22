extern crate core;

mod cli;
mod expire;

use crate::cli::{Cli, PrimaryCommands, SetSubCommands};
use crate::expire::{kill_single_expired, kill_all_expired};
use clap::Parser;
use expire::Expire;
use std::collections::{HashMap, LinkedList};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), String> {
    let mut db: HashMap<String, String> = HashMap::default();
    let mut expiring: Vec<Expire> = Vec::default();
    let mut lists: Vec<LinkedList<String>> = Vec::default();

    // redislight repl: read --> parse --> evaluate --> print -> repeat
    loop {
        let line = read_line();
        let parsed = try_parse(line);
        evaluate(parsed, &mut db, &mut expiring, &mut lists);
    }
}


/// Prompt and take user input from the command line. 
fn read_line() -> String {
    let mut line = String::new();

    print!(">> ");
    let _ = stdout().flush(); // Flushed the input buffering. Used for formatting.
    stdin().read_line(&mut line).unwrap();
    let trim = line.trim(); // Remove unwanted artifact from end and beginning of string.
    trim.to_string()
}

/// Try to parse the user's input from the command line.
fn try_parse(line: String) -> Result<Cli, String> {
    // Split the user's commands and arguments
    let args = shlex::split(&line.as_str()).ok_or("error: Invalid quoting");
    match args {
        Ok(mut split) => {
            split.insert(0, "".to_string());
            // Parse the user's split commands and arguments
            let cli = Cli::try_parse_from(&split);
            match cli {
                Ok(cli) => {
                    return Ok(cli)
                },
                Err(_) => return Err("parsing command error.".to_string())
            }
        },
        Err(_) => {
            return Err("splitting command error.".to_string());
        },
    };
}

fn evaluate(parsed: Result<Cli, String>, mut db: &mut HashMap<String, String>, mut expiring: &mut Vec<Expire>, mut lists: &mut Vec<LinkedList<String>>) {
    match parsed {
        Err(_) => {
            eprintln!("(Invalid Command)");
            return;
        }

        Ok(cli) => {
            // Look for expired pairs in the data base and remove them.
            kill_all_expired(&mut expiring, &mut db);

            match &cli.primary_commands{
                PrimaryCommands::SET {
                    key,
                    value,
                    command,
                } => {
                    // remember if new key/value pair is expiring
                    let mut is_expiring = false;

                    // consider possible SET subcommands
                    if let Some(command) = command {
                        match command {
                            // Set the specified expire time, in seconds.
                            SetSubCommands::EX { seconds } => {
                                is_expiring = true;
                                let expire =
                                    Expire::builder().key(key).seconds(seconds).finish();

                                expiring.push(expire);
                            }

                            // Set the specified expire time, in milliseconds.
                            SetSubCommands::PX { milliseconds } => {
                                is_expiring = true;
                                let expire = Expire::builder()
                                    .key(key)
                                    .milliseconds(milliseconds)
                                    .finish();

                                expiring.push(expire);
                            }

                            // Set the specified Unix time at which the key will expire, in seconds.
                            SetSubCommands::EXAT { seconds } => {
                                is_expiring = true;
                                let expire =
                                    Expire::builder().key(key).seconds(seconds).finish();

                                expiring.push(expire);
                            }

                            // Set the specified Unix time at which the key will expire, in milliseconds.
                            SetSubCommands::PXAT { milliseconds } => {
                                is_expiring = true;
                                let expire = Expire::builder()
                                    .key(key)
                                    .milliseconds(milliseconds)
                                    .finish();
                                expiring.push(expire);
                            }

                            // Keep the same key and expire, but change the value
                            SetSubCommands::KEEPTTL => {
                                db.insert(key.clone(), value.clone());
                                command_success();
                                return;
                            }

                            // Only set the key if it does not already exist.
                            SetSubCommands::NX => {
                                if !db.contains_key(key) {
                                    db.insert(key.clone(), value.clone());
                                }
                            }

                            // Only set the key if it already exist.
                            SetSubCommands::XX => {
                                if db.contains_key(key) {
                                    db.insert(key.clone(), value.clone());
                                }
                            }

                            // Return the old string stored at key, or nil if key did not exist. An error is returned and
                            // SET aborted if the value stored at key is not a string.
                            SetSubCommands::GET => {
                                if let Some(value) = db.get(key) {
                                    found_value(value)
                                } else {
                                    found_nil()
                                }
                            }
                        }
                    }

                    // If the new key/value pair is not expiring, then ensure there is no
                    // record of a previos pair with the same key set to expire.
                    if !(is_expiring) {
                        kill_single_expired(&mut expiring, &key);
                    }

                    db.insert(key.clone(), value.clone());
                    command_success();
                }
                PrimaryCommands::DEL { keys } => {
                    let mut keys_deleted = 0;
                    for key in keys {
                        if db.contains_key(key) {
                            db.remove(key);
                            keys_deleted += 1;
                        }
                    }
                    println!("(integer) {}", keys_deleted);
                }
                PrimaryCommands::GET { key } => {
                    if let Some(value) = db.get(key) {
                        found_value(value)
                    } else {
                        found_nil()
                    }
                }
                _ => {}
            }
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