use std::collections::{HashMap,HashSet};

pub struct Universe {
    inhabitants: HashMap<i8, HashSet<i8>>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe { inhabitants: HashMap::new() }
    }

    pub fn next_gen(self: &Self) -> Self {
        self.inhabitants
    }
}