use clap::{Args, Parser, Subcommand};

/// Execute Redis commands from the command line.
#[derive(Parser)]
#[command(name = "RedisLight")]
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
pub enum PrimaryCommands{
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
