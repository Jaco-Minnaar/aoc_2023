pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub use day_1::{Day1_1, Day1_2};
pub use day_2::{Day2_1, Day2_2};
pub use day_3::{Day3_1, Day3_2};
pub use day_4::{Day4_1, Day4_2};

use std::collections::HashMap;

use crate::solution::Solution;

pub struct Solutions<'a> {
    solutions: HashMap<&'a str, Box<dyn Solution>>,
}

impl<'a> Solutions<'a> {
    pub fn new() -> Self {
        Self {
            solutions: HashMap::new(),
        }
    }

    pub fn solution<S>(mut self, key: &'a str, sol: S) -> Self
    where
        S: Solution + 'static,
    {
        self.solutions.insert(key, Box::new(sol));

        self
    }

    pub fn get_solution(&self, key: &str) -> Option<&Box<dyn Solution>> {
        self.solutions.get(key)
    }
}
