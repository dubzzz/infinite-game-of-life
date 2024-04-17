mod tests;

use std::collections::HashSet;
use std::iter::Iterator;

pub struct SparseGrid {
    grid: HashSet<(isize, isize)>,
}

impl SparseGrid {
    pub fn new() -> SparseGrid {
        SparseGrid {
            grid: HashSet::new(),
        }
    }

    pub fn from(sparse_grid: &SparseGrid) -> SparseGrid {
        SparseGrid {
            grid: sparse_grid.grid.clone(),
        }
    }

    pub fn has_value(&self, row_index: isize, column_index: isize) -> bool {
        self.grid.contains(&(row_index, column_index))
    }

    pub fn append_value(&mut self, row_index: isize, column_index: isize) -> () {
        self.grid.insert((row_index, column_index));
    }

    pub fn iter(&self) -> SparseGridIter<'_> {
        SparseGridIter {
            inner_iter: Box::new(self.grid.iter().map(|data| *data)),
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
