use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::fs;

use crate::{util, ws::Meter};

/// Manages counter and should be accessible via state.
#[derive(Debug)]
pub struct Database {
    pub count: BigInt,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    /// Creates a new Database with count initialized to zero
    pub fn new() -> Self {
        Self {
            count: BigInt::zero(),
        }
    }

    /// Loads the counter from a plain text file if it exists
    pub fn load_from_file(&mut self, filename: &str) {
        if let Ok(content) = fs::read_to_string(filename) {
            if let Ok(count) = content.trim().parse::<BigInt>() {
                self.count = count;
                println!("Loaded count from file: {}", self.count);
            } else {
                println!("Invalid big.Int string in file: {}", content);
                self.count = BigInt::zero();
            }
        } else {
            println!(
                "Database file '{}' not found. Starting with zero.",
                filename
            );
        }
    }

    /// Saves the current counter to a plain text file
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        fs::write(filename, self.get_string())
    }

    /// Gets the current count as a string
    pub fn get_string(&self) -> String {
        self.count.to_str_radix(10)
    }

    /// Updates the counter based on the provided meter
    pub fn update_counter(&mut self, meter: &Meter) {
        let cmp_step = Self::compute_step(&self.count);

        let step_increment = BigInt::from(meter.increment).pow(cmp_step);

        self.count += step_increment;
        let one_googol = BigInt::parse_bytes(util::ONE_GOOGOL.as_bytes(), 10).unwrap();
        if self.count > one_googol {
            self.count = one_googol.clone();
        }

        let step_decrement = BigInt::from(meter.decrement).pow(cmp_step);

        if self.count != one_googol {
            self.count -= step_decrement.clone();
            if self.count < BigInt::zero() {
                self.count = BigInt::zero();
            }
        }
    }

    /// Function to compute the square root of the number of digits in the counter
    fn compute_step(counter: &BigInt) -> u32 {
        let abs_value = counter.abs();
        let digit_length = abs_value.to_str_radix(10).len();
        (digit_length as f64).sqrt() as u32
    }
}
