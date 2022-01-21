use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A distributed unique ID generator inspired by Twitter's Snowflake.
/// I just mimic the Twitter's Snowflake design and translate from Scala to rust.
///
/// The following url is the source. Thanks to Twitter Snowflake authors.
/// https://github.com/twitter-archive/snowflake
pub struct SnowflakeGenerator {
    /// Time cut-off
    last_time_millis: u128,

    /// Work Machine ID (0-31)
    worker_id: u8,

    /// Data Center ID (0-31)
    data_center_id: u8,

    /// Sequences in milliseconds (0-4095)
    sequence: u16,
}

impl SnowflakeGenerator {
    /// Number of digits occupied by machine id
    const WORK_ID_BITS: usize = 5;

    /// The number of digits occupied by the data identifier id
    const DATA_CENTER_BITS: usize = 5;

    /// Supported maximum machine id, the result is 31
    /// (this shift algorithm can quickly calculate the maximum decimal number represented by several bits of binary number)
    const MAX_WORK_ID: i128 = -1 ^ (-1 << SnowflakeGenerator::WORK_ID_BITS);

    /// Supported maximum data identifier id, resulting in 31
    const MAX_DATA_CENTER_ID: i128 = -1 ^ (-1 << SnowflakeGenerator::WORK_ID_BITS);

    /// Number of digits in id of sequence
    const SEQUENCE_BITS: usize = 12;

    /// The mask of the generated sequence is 4095 (0b111111111111111111111 = 0xfff = 4095)
    const SEQUENCE_MASK: i128 = -1 ^ (-1 << SnowflakeGenerator::SEQUENCE_BITS);

    /// Time truncate moves 22 bits to the left ( 5 + 5 + 12)
    const TIMESTAMP_LEFT_SHIFT: usize = SnowflakeGenerator::SEQUENCE_BITS
        + SnowflakeGenerator::WORK_ID_BITS
        + SnowflakeGenerator::DATA_CENTER_BITS;

    /// The time wanted to cut-off.
    const TIMESTAMP_OFFSET: u128 = 1577836800000;

    /// Create Snowflake generator.
    /// Please make sure that worker_id and data_center_id is between 0 and 31
    /// # Example
    ///
    /// ```
    /// let generator = SnowflakeGenerator::new(0, 0)
    /// ```
    pub fn new(worker_id: u8, data_center_id: u8) -> Self {
        if worker_id as i128 > SnowflakeGenerator::MAX_WORK_ID {
            panic!(
                "worker id can't less than 0 or bigger than {}",
                SnowflakeGenerator::MAX_WORK_ID
            );
        }

        if data_center_id as i128 > SnowflakeGenerator::MAX_DATA_CENTER_ID {
            panic!(
                "data center id can't less than 0 or bigger than {}",
                SnowflakeGenerator::MAX_DATA_CENTER_ID
            );
        }

        Self {
            last_time_millis: SnowflakeGenerator::get_current_timestamp(),
            worker_id,
            data_center_id,
            sequence: 0,
        }
    }

    /// Get the next id
    /// This function will panic if the system time has changed and the time is
    /// less than generator last_millis_timestamp
    ///
    /// # Example
    ///
    /// ```
    /// let mut generator = SnowflakeGenerator::new(0, 0)
    /// let id = generator.next_id()
    /// ```
    pub fn next_id(&mut self) -> u128 {
        let mut now = SnowflakeGenerator::get_current_timestamp();

        if now < self.last_time_millis {
            panic!(
                "Clock moved backwards, Refusing to generate id for {} milliseconds",
                self.last_time_millis - now
            );
        }

        if self.last_time_millis == now {
            self.sequence =
                (((self.sequence + 1) as i128) % SnowflakeGenerator::SEQUENCE_MASK) as u16;
            if self.sequence == 0 {
                now = SnowflakeGenerator::til_next_millis(self.last_time_millis);
            }
        } else {
            self.sequence = 0;
        }

        self.last_time_millis = now;

        (self.last_time_millis << SnowflakeGenerator::TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << SnowflakeGenerator::DATA_CENTER_BITS) as u128
            | (self.worker_id << SnowflakeGenerator::WORK_ID_BITS) as u128
            | self.sequence as u128
    }

    /// Block to the next millisecond until a new timestamp is obtained. (Private function)
    fn til_next_millis(last_time_millis: u128) -> u128 {
        loop {
            let now = SnowflakeGenerator::get_current_timestamp();
            if now > last_time_millis {
                return now;
            }
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    /// Get the current timestamp in millisecond. (Private function)
    fn get_current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't get current timestamp")
            .as_millis() - SnowflakeGenerator::TIMESTAMP_OFFSET
    }
}