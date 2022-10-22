use std::{time::{Duration, Instant}, collections::HashMap};

/// Associates a key with a given expiration timer.
#[derive(Debug)]
pub struct Expire {
    pub key: String,
    pub time: Duration,
    pub birth: Instant,
}

impl Expire {
    /// Discover's the builder to user configuration.
    pub fn builder() -> ExpireBuilder {
        ExpireBuilder::default()
    }
}

#[derive(Default)]
pub struct ExpireBuilder {
    key: Option<String>,
    time: Option<Duration>,
    birth: Option<Instant>,
}

impl ExpireBuilder {
    pub fn key(mut self, key: &String) -> Self {
        self.key = Some(key.clone());
        return self;
    }

    pub fn seconds(mut self, sec: &f32) -> Self {
        self.time = Some(Duration::from_secs_f32(*sec));
        self
    }

    pub fn milliseconds(mut self, millis: &u64) -> Self {
        self.time = Some(Duration::from_millis(*millis));
        self
    }

    /// Returns the build Expire; requires a key and a time.
    pub fn finish(self) -> Expire {
        Expire {
            key: self.key.unwrap(),
            time: self.time.unwrap(),
            birth: Instant::now(),
        }
    }
}

/// Calculates the difference of ((birth + time) - current). Removes
/// key from the data base if result is negative.
pub fn kill_all_expired(expiring: &mut Vec<Expire>, hash: &mut HashMap<String, String>) {
    let mut killing: Vec<usize> = Vec::default();

    // Remove dead pairs from Database
    for (i, expire) in expiring.iter().enumerate() {
        let diff = (expire.birth + expire.time) - Instant::now();
        if diff <= Duration::ZERO {
            hash.remove(&expire.key);
            killing.push(i);
        }
    }

    // Remove dead Expires from Expiring List
    for i in killing {
        println!("{:?}", expiring.get(i));
        expiring.remove(i);
    }
}

/// Kills specified key/value pair from the expiring list.
pub fn kill_single_expired(expiring: &mut Vec<Expire>, key: &String) {
    let mut delete: Option<usize> = None;
    for (i, expire) in expiring.iter().enumerate() {
        if expire.key.eq(key) {
            delete = Some(i);
            break;
        }
    }

    // Kill from expiring list.
    if let Some(i) = delete {
        expiring.remove(i);
    }
}