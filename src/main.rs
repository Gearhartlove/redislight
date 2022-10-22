extern crate core;

mod cli;
mod expire;

use crate::cli::{Cli, Commands, SetCommands};
use crate::expire::{kill_single_expired, kill_all_expired};
use clap::Parser;
use expire::Expire;
use std::collections::{HashMap, LinkedList};
use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let mut db: HashMap<String, String> = HashMap::default();
    let mut expiring: Vec<Expire> = Vec::default();
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
        let cli = Cli::try_parse_from(&args);

        match cli {
            Err(_) => {
                eprintln!("(Invalid Command)");
                continue;
            }

            Ok(cli) => {
                // Look for expired pairs in the data base and remove them.
                kill_all_expired(&mut expiring, &mut db);

                match &cli.command {
                    Commands::SET {
                        key,
                        value,
                        command,
                    } => {
                        // remember if new key/value pair is expiring
                        let mut is_expiring = false;

                        // consider possible SET subcommands
                        if let Some(command) = command {
                            match command {
                                SetCommands::EX { seconds } => {
                                    is_expiring = true;
                                    let expire =
                                        Expire::builder().key(key).seconds(seconds).finish();

                                    expiring.push(expire);
                                }
                                SetCommands::PX { milliseconds } => {
                                    is_expiring = true;
                                    let expire = Expire::builder()
                                        .key(key)
                                        .milliseconds(milliseconds)
                                        .finish();

                                    expiring.push(expire);
                                }
                                SetCommands::EXAT { seconds } => {
                                    is_expiring = true;
                                    let expire =
                                        Expire::builder().key(key).seconds(seconds).finish();

                                    expiring.push(expire);
                                }
                                SetCommands::PXAT { milliseconds } => {
                                    is_expiring = true;
                                    let expire = Expire::builder()
                                        .key(key)
                                        .milliseconds(milliseconds)
                                        .finish();
                                    expiring.push(expire);
                                }
                                // Keep the same key and expire, but change the value
                                SetCommands::KEEPTTL => {
                                    db.insert(key.clone(), value.clone());
                                    command_success();
                                    continue;
                                }
                                SetCommands::NX => {
                                    if !db.contains_key(key) {
                                        db.insert(key.clone(), value.clone());
                                    }
                                }
                                SetCommands::XX => {
                                    if db.contains_key(key) {
                                        db.insert(key.clone(), value.clone());
                                    }
                                }
                                SetCommands::GET => {
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
                    Commands::DEL { keys } => {
                        let mut keys_deleted = 0;
                        for key in keys {
                            if db.contains_key(key) {
                                db.remove(key);
                                keys_deleted += 1;
                            }
                        }
                        println!("(ingeger) {}", keys_deleted);
                    }
                    Commands::GET { key } => {
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