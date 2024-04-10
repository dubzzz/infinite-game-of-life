#[cfg(test)]
mod has_value {
    use crate::universe::sparse_grid::SparseGrid;

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
}

#[cfg(test)]
mod iter {
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
}
