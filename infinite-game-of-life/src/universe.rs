use std::collections::{HashMap, HashSet};

pub struct Universe {
    inhabitants: HashMap<i8, HashSet<i8>>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            inhabitants: HashMap::new(),
        }
    }

    fn has_value(data: &HashMap<i8, HashSet<i8>>, row_index: i8, column_index: i8) -> bool {
        data.get(&row_index)
            .map(|row| row.contains(&column_index))
            .unwrap_or_default()
    }

    fn append_value(data: &mut HashMap<i8, HashSet<i8>>, row_index: i8, column_index: i8) -> () {
        let row = data.get_mut(&row_index);
        match row {
            Some(row) => {
                row.insert(column_index);
            }
            None => {
                let mut row = HashSet::new();
                row.insert(column_index);
                data.insert(row_index, row);
            }
        }
    }

    fn is_alive(self: &Self, row_index: i8, column_index: i8) -> bool {
        Universe::has_value(&self.inhabitants, row_index, column_index)
    }

    fn neighborhood(row_index: i8, column_index: i8) -> Vec<(i8, i8)> {
        vec![
            (
                row_index.overflowing_sub(1).0,
                column_index.overflowing_sub(1).0,
            ),
            (row_index.overflowing_sub(1).0, column_index),
            (
                row_index.overflowing_sub(1).0,
                column_index.overflowing_add(1).0,
            ),
            (row_index, column_index.overflowing_sub(1).0),
            (row_index, column_index),
            (row_index, column_index.overflowing_add(1).0),
            (
                row_index.overflowing_add(1).0,
                column_index.overflowing_sub(1).0,
            ),
            (row_index.overflowing_add(1).0, column_index),
            (
                row_index.overflowing_add(1).0,
                column_index.overflowing_add(1).0,
            ),
        ]
    }

    pub fn next_gen(self: &Self) -> Self {
        let mut new_inhabitants = HashMap::new();
        let mut already_scanned = HashMap::new();
        for (row_index, row) in self.inhabitants.iter() {
            for column_index in row {
                for (new_row_index, new_column_index) in
                    Self::neighborhood(*row_index, *column_index)
                {
                    if !Self::has_value(&already_scanned, new_row_index, new_column_index) {
                        Self::append_value(&mut already_scanned, new_row_index, new_column_index);
                        let current_alive = self.is_alive(new_row_index, new_column_index);
                        let num_alives = Self::neighborhood(*row_index, *column_index)
                            .iter()
                            .filter(|(r, c)| self.is_alive(*r, *c))
                            .count();
                        if current_alive && (num_alives == 3 || num_alives == 4) {
                            Self::append_value(&mut new_inhabitants, new_row_index, new_column_index);
                        } else if !current_alive && num_alives == 3 {
                            Self::append_value(&mut new_inhabitants, new_row_index, new_column_index);
                        }
                    }
                }
            }
        }
        Universe {
            inhabitants: new_inhabitants,
        }
    }
}
