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

    pub fn window<T: Clone>(
        &self,
        row_index_start: i8,
        column_index_start: i8,
        row_index_end: i8,
        column_index_end: i8,
        with_value: T,
        without_value: T,
    ) -> Vec<Vec<T>> {
        let mut content = Vec::new();
        for row_index in row_index_start..=row_index_end {
            let mut content_row = Vec::new();
            for column_index in column_index_start..=column_index_end {
                if self.has_value(row_index, column_index) {
                    content_row.push(with_value.clone())
                } else {
                    content_row.push(without_value.clone())
                }
            }
            content.push(content_row);
        }
        content
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

#[cfg(test)]
mod tests {
    use crate::universe::sparse_grid::SparseGrid;
    use std::cmp::Ordering;

    fn sort_tuples(t1: (i8, i8), t2: (i8, i8)) -> Ordering {
        if t1.0 == t2.0 && t1.1 == t2.1 {
            Ordering::Equal
        } else if t1.0 < t2.0 || (t1.0 == t2.0 && t1.1 < t2.1) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    #[test]
    fn should_not_find_value_in_empty_grid() {
        let grid = SparseGrid::new();
        assert_eq!(grid.has_value(0, 0), false);
    }

    #[test]
    fn should_not_find_value_if_not_in_grid() {
        let mut grid = SparseGrid::new();
        grid.append_value(-1, 0);
        assert_eq!(grid.has_value(0, 0), false);
    }

    #[test]
    fn should_find_value_if_in_grid() {
        let mut grid = SparseGrid::new();
        grid.append_value(0, 0);
        assert_eq!(grid.has_value(0, 0), true);
    }

    #[test]
    fn should_be_able_to_iterate_on_all_values() {
        let mut grid = SparseGrid::new();
        grid.append_value(-1, 0);
        grid.append_value(-1, 1);
        grid.append_value(2, 1);
        grid.append_value(2, 5);
        grid.append_value(3, 10);
        let mut iterated: Vec<_> = grid.iter().collect();
        let mut expected = vec![(-1, 0), (-1, 1), (2, 1), (2, 5), (3, 10)];
        iterated.sort_by(|t1, t2| sort_tuples(*t1, *t2));
        expected.sort_by(|t1, t2| sort_tuples(*t1, *t2));
        assert_eq!(iterated.sort(), expected.sort());
    }

    #[test]
    fn should_be_able_to_export_a_window_on_part_data() {
        let mut grid = SparseGrid::new();
        grid.append_value(-1, 0);
        grid.append_value(-1, 1);
        grid.append_value(2, 1);
        grid.append_value(2, 5);
        grid.append_value(3, 10);
        let expected = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];
        assert_eq!(grid.window(0, 0, 4, 4, true, false), expected);
    }

    #[test]
    fn should_be_able_to_export_a_negative_window_on_part_data() {
        let mut grid = SparseGrid::new();
        grid.append_value(-1, 0);
        grid.append_value(-1, 1);
        grid.append_value(2, 1);
        grid.append_value(2, 5);
        grid.append_value(3, 10);
        let expected = vec![
            vec![false, false, true, true, false, false],
            vec![false, false, false, false, false, false],
            vec![false, false, false, false, false, false],
            vec![false, false, false, true, false, false],
        ];
        assert_eq!(grid.window(-1, -2, 2, 3, true, false), expected);
    }
}
