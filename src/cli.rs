use clap::{Args, Parser, Subcommand};

/// Execute Redis commands from the command line.
#[derive(Parser)]
#[command(name = "redislight")]
#[command(author = "Gearhartlove")]
#[command(version = "1.0")]
#[command(about = "Mimics basic Redis commands in the console.", long_about = None)]
pub struct Cli {
    /// Commands to be executed.
    #[command(subcommand)]
    pub primary_commands: PrimaryCommands,
}

#[derive(Subcommand)]
#[command(rename_all = "UPPER")]
pub enum PrimaryCommands {
    /// Set 'key' to hold the string 'value'.
    SET {
        /// Determines where in the table the data will be stored.
        #[arg(value_name = "KEY")]
        key: String,
        /// Determines what will be stored at a given table location.
        #[arg(value_name = "VALUE")]
        value: String,
        /// Request information about a given key.
        #[command(subcommand)]
        command: Option<SetSubCommands>,
    },
    /// Removes the specified keys. A key is ignored if it does not exist.
    DEL {
        /// Keys to be removed.
        #[arg(value_name = "KEYS")]
        keys: Vec<String>,
    },
    /// Get the value of key. If the key does not exist the special value nil is returned. An error
    /// is returned if the value stored at key is not a string, because GET only handles string values.
    GET {
        /// Key to get.
        #[arg(value_name = "KEY")]
        key: String,
    },
    /// Insert all the specified values at the head of the list stored at key. If key does not exist,
    /// it is created as empty list before performing the push operations. When key holds a value that
    /// is not a list, an error is returned.
    LPUSH {
        /// Key from which specified elements are pushed.
        #[arg(value_name = "KEY")]
        key: String,
        /// List of elements to add to list.
        #[arg(value_name = "ELEMENT")]
        add_elements: Vec<String>,
    },
    /// Removes and returns the first elements of the list stored at key.
    LPOP {
        /// Key from which specified elements are popped.
        #[arg(value_name = "KEY")]
        key: String,
        /// List of elements to pop from to list.
        #[arg(value_name = "COUNT")]
        count: Option<usize>,
    },
    /// Returns the specified elements of the list stored at key. The offsets start and stop are zero-based
    /// indexes, with 0 being the first element of the list (the head of the list), 1 being the next element
    /// and so on.
    LRANGE {
        /// Key from which specified elements are returned.
        #[arg(value_name = "KEY")]
        key: String,
        /// Start offset for zero-based indexes.
        #[arg(value_name = "START")]
        start: usize,
        /// End offset for zero-based indexes.
        #[arg(value_name = "STOP")]
        stop: usize,
    },
}

#[allow(non_camel_case_types)]
#[derive(Subcommand, Clone, Debug)]
#[command(rename_all = "UPPER")]
pub enum SetSubCommands {
    /// Set the specified expire time, in seconds.
    EX { seconds: f32 },
    /// Set the specified expire time, in milliseconds.
    PX { milliseconds: u64 },
    /// Set the specified Unix time at which the key will expire, in seconds.
    EXAT { seconds: f32 },
    /// Set the specified Unix time at which the key will expire, in milliseconds.
    PXAT { milliseconds: u64 },
    /// Only set the key if it does not already exist.
    NX,
    /// Only set the key if it already exist.
    XX,
    /// Retain the time to live associated with the key.
    KEEPTTL,
    /// Return the old string stored at key, or nil if key did not exist. An error is returned and
    /// SET aborted if the value stored at key is not a string.
    GET,
}
