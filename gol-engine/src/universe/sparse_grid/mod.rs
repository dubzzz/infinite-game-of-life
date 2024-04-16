mod tests;

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

pub struct SparseGrid {
    grid: HashMap<isize, HashSet<isize>>,
}

impl SparseGrid {
    pub fn new() -> SparseGrid {
        SparseGrid {
            grid: HashMap::new(),
        }
    }

    pub fn from(sparse_grid: &SparseGrid) -> SparseGrid {
        SparseGrid {
            grid: sparse_grid.grid.clone(),
        }
    }

    pub fn has_value(&self, row_index: isize, column_index: isize) -> bool {
        self.grid
            .get(&row_index)
            .map(|row| row.contains(&column_index))
            .unwrap_or_default()
    }

    pub fn append_value(&mut self, row_index: isize, column_index: isize) -> () {
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
    inner_iter: Box<dyn Iterator<Item = (isize, isize)> + 'a>,
}

impl<'a> Iterator for SparseGridIter<'a> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}
