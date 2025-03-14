use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use tracing::info;

use crate::{
    util,
    ws::{CounterState, PollState},
};

/// Manages counter and should be accessible via state.
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Counter {
    pub count: Count,
    pub poll: Option<Poll>,
    pub upgrade: Upgrade,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    /// Creates a new Counter with count initialized to zero
    pub fn new() -> Self {
        Self {
            count: Count::default(),
            poll: None,
            upgrade: Upgrade::default(),
        }
    }

    /// Loads the counter from a plain text file if it exists
    pub fn load_from_file(&mut self, path: &PathBuf) {
        if path.exists() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            let counter: Self = serde_json::from_reader(reader).unwrap();
            *self = counter;
            info!("Loaded Counter from file: {:?}", path);
        } else {
            self.save_to_file(path).unwrap();
            info!("Created Counter at: {:?}", path);
        }
    }

    /// Saves the current counter to a plain text file
    pub fn save_to_file(&self, path: &PathBuf) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Gets the current count as a string
    pub fn count_string(&self) -> String {
        self.count.value.to_str_radix(10)
    }

    /// Updates the counter based on the provided counter states
    pub fn update_count(&mut self, counter_all: &[(CounterState, usize)]) {
        let mut counter_states = vec![];
        self.count.accumulated_actions = 0;

        for (counter_state, action_clicks) in counter_all {
            counter_states.push(counter_state.clone());
            let action_clicks = (*action_clicks) as i64;
            if counter_state == &CounterState::Increment {
                self.count.accumulated_actions += action_clicks;
            } else if counter_state == &CounterState::Decrement {
                self.count.accumulated_actions -= action_clicks;
            }
        }

        self.count.meter = CounterState::meter_counter(&counter_states);

        let cmp_step = Self::compute_step(&self.count.value);

        // Compute the modifier in BigInt to avoid overflow
        let actions_modifier =
            BigInt::from(self.count.accumulated_actions) * self.upgrade.modifier(cmp_step);
        let positive_modifier = if actions_modifier > BigInt::zero() {
            actions_modifier.clone()
        } else {
            BigInt::zero()
        };
        let negative_modifier = if actions_modifier < BigInt::zero() {
            -actions_modifier.clone()
        } else {
            BigInt::zero()
        };

        let step_increment = (BigInt::from(self.count.meter.increment)
            * BigInt::from(self.upgrade.base))
        .pow(cmp_step + self.upgrade.exponent)
            + positive_modifier;

        self.count.value += step_increment;
        let one_googol = BigInt::parse_bytes(util::ONE_GOOGOL.as_bytes(), 10).unwrap();
        if self.count.value > one_googol {
            self.count.value = one_googol.clone();
        }

        let step_decrement = (BigInt::from(self.count.meter.decrement)
            * BigInt::from(self.upgrade.base))
        .pow(cmp_step + self.upgrade.exponent)
            + negative_modifier;

        if self.count.value != one_googol {
            self.count.value -= step_decrement;
            if self.count.value < BigInt::zero() {
                self.count.value = BigInt::zero();
            }
        }
    }

    /// Function to compute the square root of the number of digits in the counter
    fn compute_step(counter: &BigInt) -> u32 {
        let abs_value = counter.abs();
        let digit_length = abs_value.to_str_radix(10).len();
        (digit_length as f64).sqrt() as u32
    }

    /// Updates the poll based on the provided poll states
    pub fn update_poll(&mut self, poll_states: &[PollState]) {
        if self.is_at_upgrade() {
            if let Some(poll) = &mut self.poll {
                poll.amplification += 1;
            } else {
                self.poll = Some(Poll::new());
            }
        }

        if let Some(poll) = &mut self.poll {
            poll.meter = PollState::meter_poll(poll_states);

            if poll.tick() {
                if poll.meter.base > poll.meter.exponent {
                    self.upgrade.base += poll.amplification;
                    self.upgrade.last_upgrade = PollState::Base;
                } else {
                    self.upgrade.exponent += poll.amplification;
                    self.upgrade.last_upgrade = PollState::Exponent;
                }
                // reset poll
                self.poll = None;
            }
        }
    }

    /// Returns true if at an upgrade and increases the upgrade level accordingly
    fn is_at_upgrade(&mut self) -> bool {
        let length = self.count_string().len();
        let level = (length + 8) / 10;
        if level > self.upgrade.level {
            self.upgrade.level = level;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Count {
    #[serde(
        serialize_with = "util::serialize_bigint",
        deserialize_with = "util::deserialize_bigint"
    )]
    pub value: BigInt,
    pub meter: CountMeter,
    pub accumulated_actions: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Poll {
    pub time_remaining: u32,
    pub amplification: u32,
    pub meter: PollMeter,
}

impl Default for Poll {
    fn default() -> Self {
        Self::new()
    }
}

impl Poll {
    pub fn new() -> Self {
        Self {
            time_remaining: 1200, // 300s * 4 -> 1/4s
            amplification: 1,
            meter: PollMeter::default(),
        }
    }

    /// Ticks a tick and returns true if `time_remaining == 0`
    pub fn tick(&mut self) -> bool {
        if self.time_remaining == 0 {
            return true;
        } else if self.meter.base == self.meter.exponent {
            return false;
        }

        if self.meter.pending < self.meter.base + self.meter.exponent {
            self.time_remaining = self.time_remaining.saturating_sub(4);
        } else {
            self.time_remaining = self.time_remaining.saturating_sub(1);
        }
        false
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Upgrade {
    pub level: usize,
    pub last_upgrade: PollState,
    pub base: u32,
    pub exponent: u32,
}

impl Default for Upgrade {
    fn default() -> Self {
        Self::new()
    }
}

impl Upgrade {
    pub fn new() -> Self {
        Self {
            level: 0,
            last_upgrade: PollState::default(),
            base: 1,
            exponent: 0,
        }
    }

    // Use BigInt for the modifier to avoid fixed-size integer overflow.
    pub fn modifier(&self, cmp_step: u32) -> BigInt {
        let factor = BigInt::from(self.level as i64 + 1) * BigInt::from(self.base as i64);
        let exp = cmp_step + self.exponent;
        factor.pow(exp)
    }
}

/// Client counter state count
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountMeter {
    pub increment: u32,
    pub decrement: u32,
    pub pending: u32,
}

/// Client poll state count
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PollMeter {
    pub base: u32,
    pub exponent: u32,
    pub pending: u32,
}
