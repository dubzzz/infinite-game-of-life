mod tests;

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

pub struct SparseGrid {
    grid: HashMap<i8, HashSet<i8>>,
}

impl SparseGrid {
    pub fn new() -> SparseGrid {
        SparseGrid {
            grid: HashMap::new(),
        }
    }

    pub fn has_value(&self, row_index: i8, column_index: i8) -> bool {
        self.grid
            .get(&row_index)
            .map(|row| row.contains(&column_index))
            .unwrap_or_default()
    }

    pub fn append_value(&mut self, row_index: i8, column_index: i8) -> () {
        let row = self.grid.get_mut(&row_index);
        match row {
            Some(row) => {
                row.insert(column_index);
            }
            None => {
                let mut row = HashSet::new();
                row.insert(column_index);
                self.grid.insert(row_index, row);
            }
        }
    }

    pub fn iter(&self) -> SparseGridIter<'_> {
        SparseGridIter {
            inner_iter: Box::new(
                self.grid
                    .iter()
                    .flat_map(|(row, columns)| columns.iter().map(move |&col| (*row, col))),
            ),
        }
    }
}

pub struct SparseGridIter<'a> {
    inner_iter: Box<dyn Iterator<Item = (i8, i8)> + 'a>,
}

impl<'a> Iterator for SparseGridIter<'a> {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}
