#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct SnowflakeGenerator {
    epoch: SystemTime,
    last_time: u64,
    machine_id: u8,
    thread_id: u8,
    counter: u16,
}

#[derive(Debug)]
pub struct Snowflake {
    /// The time in milliseconds since the epoch.
    /// This field does not automatically compensate if an epoc other than UNIX_EPOCH is used.
    pub time: u64,
    /// The machine ID the snowflake was generated on.
    pub machine_id: u8,
    /// The thread ID the snowflake was generated on.
    pub thread_id: u8,
    /// The counter for the snowflake. This is incremented every time a snowflake is generated
    /// and reset if the time has changed
    pub counter: u16,
}

impl SnowflakeGenerator {
    /// Creates a new SnowflakeGenerator with the given machine ID and thread ID.
    /// The machine ID must be less than 32 and the thread ID must be less than 32.
    ///
    /// # Examples
    /// ```
    /// # use snowflake::SnowflakeGenerator;
    /// let mut generator = SnowflakeGenerator::new(0, 0);
    /// ```
    /// # Panics
    /// This function will panic if the machine ID or thread ID is greater than 31.
    ///
    /// ```should_panic
    /// # use snowflake::SnowflakeGenerator;
    /// let mut generator = SnowflakeGenerator::new(32, 32);
    /// ```
    pub fn new(machine_id: u8, thread_id: u8) -> Self {
        SnowflakeGenerator::with_epoch(UNIX_EPOCH, machine_id, thread_id)
    }

    /// Creates a new SnowflakeGenerator with the given epoch, machine ID, and thread ID.
    /// The machine ID must be less than 32 and the thread ID must be less than 32.
    /// The epoch is the time that the SnowflakeGenerator will use as the start of time.
    /// This is useful if you want to use a different epoch than the Unix epoch.
    ///
    /// # Examples
    /// ```
    /// # use snowflake::SnowflakeGenerator;
    /// # use std::time::UNIX_EPOCH;
    /// let mut generator = SnowflakeGenerator::with_epoch(UNIX_EPOCH, 0, 0);
    /// ```
    ///
    /// # Panics
    /// This function will panic if the machine ID or thread ID is greater than 31.
    ///
    /// ```should_panic
    /// # use snowflake::SnowflakeGenerator;
    /// # use std::time::UNIX_EPOCH;
    /// let mut generator = SnowflakeGenerator::with_epoch(UNIX_EPOCH, 32, 32);
    /// ```
    pub fn with_epoch(epoch: SystemTime, machine_id: u8, thread_id: u8) -> Self {
        assert!(machine_id < 32, "machine_id must be less than 32");
        assert!(thread_id < 32, "thread_id must be less than 32");
        SnowflakeGenerator {
            epoch,
            last_time: get_time_millis(epoch),
            machine_id,
            thread_id,
            counter: 0,
        }
    }

    /// Generates a new Snowflake ID.
    /// This function will block until it can generate a new ID.
    ///
    /// # Examples
    /// ```
    /// # use snowflake::SnowflakeGenerator;
    /// let mut generator = SnowflakeGenerator::new(0, 0);
    /// let id = generator.generate();
    /// ```
    pub fn generate(&mut self) -> u64 {
        let mut now = get_time_millis(self.epoch);

        // If the time is the same as the last time we generated an ID, we need to increment our counter
        if now == self.last_time {
            self.counter = (self.counter + 1) % 4096;
            if self.counter == 0 {
                // If we've reached the maximum number of IDs we can generate in a single millisecond,
                // we need to wait until the next millisecond
                while now <= self.last_time {
                    now = get_time_millis(self.epoch);
                }
            }
        } else {
            // This is a new millisecond so we reset our counter
            self.counter = 0;
        }

        self.last_time = now;

        self.last_time << 22
            | ((self.machine_id as u64) << 17)
            | ((self.thread_id as u64) << 12)
            | (self.counter as u64)
    }

    /// Generates a new Snowflake ID.
    /// This function will not block and will increment the timestamp if the counter is full.
    ///
    /// # Examples
    /// ```
    /// # use snowflake::SnowflakeGenerator;
    /// let mut generator = SnowflakeGenerator::new(0, 0);
    /// let id = generator.generate_fuzzy();
    /// ```
    pub fn generate_fuzzy(&mut self) -> u64 {
        let mut now = get_time_millis(self.epoch);

        // If the actual time is less than or the same as the last time we generated an ID,
        // we need to increment our counter
        if now <= self.last_time {
            self.counter = (self.counter + 1) % 4096;
            if self.counter == 0 {
                // If we've reached the maximum number of IDs we can generate in a single millisecond,
                // we need to increment the current millisecond
                now += 1;
            }
        } else {
            // This is a new millisecond so we reset our counter
            self.counter = 0;
        }

        self.last_time = now;

        self.last_time << 22
            | ((self.machine_id as u64) << 17)
            | ((self.thread_id as u64) << 12)
            | (self.counter as u64)
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Snowflake {
            time: value >> 22,
            machine_id: ((value & 0x3E0000) >> 17) as u8,
            thread_id: ((value & 0x1F000) >> 12) as u8,
            counter: (value & 0xFFF) as u16,
        }
    }
}
impl From<Snowflake> for u64 {
    fn from(value: Snowflake) -> Self {
        value.time << 22
            | ((value.machine_id as u64) << 17)
            | ((value.thread_id as u64) << 12)
            | (value.counter as u64)
    }
}

fn get_time_millis(epoch: SystemTime) -> u64 {
    SystemTime::now()
        .duration_since(epoch)
        .expect("time is before epoch")
        .as_millis() as u64
}
