extern crate core;

mod cli;
mod expire;
mod value;

use crate::cli::{Cli, PrimaryCommands, SetSubCommands};
use crate::expire::{kill_all_expired, kill_single_expired};
use clap::Parser;
use expire::Expire;
use std::collections::{HashMap, LinkedList};
use std::io::{stdin, stdout, Write};
use value::Value;

fn main() -> Result<(), String> {
    let mut db: HashMap<String, Value> = HashMap::default();
    let mut expiring: Vec<Expire> = Vec::default();

    // redislight repl: read --> parse --> evaluate --> print -> repeat
    loop {
        let line = read_line();
        let parsed = try_parse(line);
        evaluate(parsed, &mut db, &mut expiring);
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
                Ok(cli) => return Ok(cli),
                Err(_) => return Err("parsing command error.".to_string()),
            }
        }
        Err(_) => {
            return Err("splitting command error.".to_string());
        }
    };
}

/// Interpret the user's parced command statement with optional args and subcommands.
fn evaluate(
    parsed: Result<Cli, String>,
    mut db: &mut HashMap<String, Value>,
    mut expiring: &mut Vec<Expire>,
) {
    match parsed {
        Err(_) => {
            eprintln!("(invalid command)");
            return;
        }

        Ok(cli) => {
            // Look for expired pairs in the data base and remove them.
            kill_all_expired(&mut expiring, &mut db);

            match &cli.primary_commands {
                // Set key to hold the string value.
                PrimaryCommands::SET {
                    key,
                    value,
                    command,
                } => {
                    // remember if new key/value pair is expiring
                    let mut is_expiring = false;
                    // create owned verison of value
                    let value = Value::Str(value.clone());

                    // consider possible SET subcommands
                    if let Some(command) = command {
                        match command {
                            // Set the specified expire time, in seconds.
                            SetSubCommands::EX { seconds } => {
                                is_expiring = true;
                                let expire = Expire::builder().key(key).seconds(seconds).finish();

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
                                let expire = Expire::builder().key(key).seconds(seconds).finish();

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
                                db.insert(key.clone(), value);
                                command_success();
                                return;
                            }

                            // Only set the key if it does not already exist.
                            SetSubCommands::NX => {
                                if !db.contains_key(key) {
                                    // Remove existing expire if present
                                    kill_single_expired(&mut expiring, &key);
                                    db.insert(key.clone(), value);
                                    return;
                                }
                            }

                            // Only set the key if it already exist.
                            SetSubCommands::XX => {
                                if db.contains_key(key) {
                                    // Remove existing expire if present
                                    kill_single_expired(&mut expiring, &key);
                                    db.insert(key.clone(), value);
                                    return;
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

                    db.insert(key.clone(), value);
                    command_success();
                }
                // Removes the specified keys. A key is ignored if it does not exist.
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
                // Get the value of key.
                PrimaryCommands::GET { key } => {
                    if let Some(value) = db.get(key) {
                        found_value(value)
                    } else {
                        found_nil()
                    }
                }
                // Insert all the specified values at the head of the list stored at key.
                PrimaryCommands::LPUSH { key, add_elements } => {
                    // Get the linked list to add the elements too. If it doesn't exit, instantiate it.
                    let value = match db.get_mut(key) {
                        Some(ll) => ll,
                        None => {
                            let ll: LinkedList<String> = LinkedList::default();
                            let value = Value::LL(ll);
                            db.insert(key.clone(), value);
                            db.get_mut(key).unwrap()
                        }
                    };

                    match value {
                        Value::LL(ll) => {
                            for element in add_elements.iter() {
                                ll.push_front(element.clone());
                            }

                            println!("(integer) {}", ll.len())
                        }
                        _ => {}
                    }
                }
                // Removes and returns the first elements of the list stored at key.
                PrimaryCommands::LPOP { key, count } => {
                    // Get the linked list from the database if it exists.
                    if let Some(value) = db.get_mut(key) {
                        match value {
                            // Get the linked list.
                            Value::LL(ll) => {
                                // Create popped vector for future popped values to be added to.
                                let mut popped: Vec<Option<String>> = Vec::default();
                                match count {
                                    Some(count) => {
                                        for _ in 0..*count {
                                            let pop = ll.pop_front();
                                            popped.push(pop);
                                        }
                                    }
                                    // Default behavior: pop one from the start if no count is specified.
                                    None => {
                                        let pop = ll.pop_front();
                                        popped.push(pop);
                                    }
                                }

                                // If a value was popped, print that value.
                                for (i, pop) in popped.iter().enumerate() {
                                    if let Some(value) = pop {
                                        println!("{}) {}", i + 1, value)
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                // Returns the specified elements of the list stored at key.
                PrimaryCommands::LRANGE { key, start, stop } => {
                    // Get the linked list from the database if it exists.
                    if let Some(value) = db.get(key) {
                        match value {
                            Value::LL(ll) => {
                                // From the end to the start, if there is a value, return that value.
                                for i in *start..*stop + 2 {
                                    if let Some(value) = ll.iter().nth(i) {
                                        println!("{}) {}", i + 1, value);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// Utility functions

fn found_value(value: &Value) {
    match value {
        Value::Str(s) => {
            println!("{}", s)
        },
        Value::LL(ll) => {
            for (i, s) in ll.iter().enumerate() {
                println!("{}) {}", i + 1, s)
            }
        },
    }
}

fn command_success() {
    println!("OK");
}

fn found_nil() {
    println!("(nil)")
}
