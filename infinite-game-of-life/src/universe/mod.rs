mod sparse_grid;
use sparse_grid::SparseGrid;

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Dead,
    Alive,
}

pub struct Universe {
    inhabitants: SparseGrid,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            inhabitants: SparseGrid::new(),
        }
    }

    fn set_alive(self: &mut Self, row_index: i8, column_index: i8) -> () {
        self.inhabitants.append_value(row_index, column_index)
    }

    fn is_alive(self: &Self, row_index: i8, column_index: i8) -> bool {
        self.inhabitants.has_value(row_index, column_index)
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
        let mut new_inhabitants = SparseGrid::new();
        let mut already_scanned = SparseGrid::new();

        let possible_inhabitants = self
            .inhabitants
            .iter()
            .flat_map(|inhabitant| Self::neighborhood(inhabitant.0, inhabitant.1));
        for (row_index, column_index) in possible_inhabitants {
            if already_scanned.has_value(row_index, column_index) {
                continue;
            }
            already_scanned.append_value(row_index, column_index);

            let current_alive = self.is_alive(row_index, column_index);
            let num_alives = Self::neighborhood(row_index, column_index)
                .iter()
                .filter(|(r, c)| self.is_alive(*r, *c))
                .count();
            if current_alive && (num_alives == 3 || num_alives == 4) {
                new_inhabitants.append_value(row_index, column_index);
            } else if !current_alive && num_alives == 3 {
                new_inhabitants.append_value(row_index, column_index);
            }
        }
        Universe {
            inhabitants: new_inhabitants,
        }
    }

    pub fn window(
        &self,
        row_index_start: i8,
        column_index_start: i8,
        row_index_end: i8,
        column_index_end: i8,
    ) -> Vec<Vec<Status>> {
        self.inhabitants.window(
            row_index_start,
            column_index_start,
            row_index_end,
            column_index_end,
            Status::Alive,
            Status::Dead,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Status, Universe};

    #[test]
    fn should_converge_to_a_square() {
        let mut universe = Universe::new();
        universe.set_alive(1, 1);
        universe.set_alive(1, 2);
        universe.set_alive(2, 1);
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);
    }

    #[test]
    fn should_converge_to_nothing_after_passing_to_one_cell() {
        let mut universe = Universe::new();
        universe.set_alive(1, 1);
        universe.set_alive(1, 3);
        universe.set_alive(3, 1);
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Alive],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(0, 0, 5, 4), expected_universe);
    }

    #[test]
    fn should_handle_highly_spread_envs() {
        let mut universe = Universe::new();
        universe.set_alive(i8::min_value(), i8::min_value());
        universe.set_alive(0, 0);
        universe.set_alive(i8::max_value(), i8::max_value());
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Dead, Status::Alive, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, ],
        ];
        assert_eq!(universe.window(-1,-1,2,2), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, ],
        ];
        assert_eq!(universe.window(-1,-1,2,2), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(-1,-1,2,2), expected_universe);
    }

    #[test]
    fn should_join_the_extreme_parts() {
        let mut universe = Universe::new();
        // left branch
        universe.set_alive(i8::min_value(), i8::max_value()-3);
        universe.set_alive(i8::min_value(), i8::max_value()-2);
        universe.set_alive(i8::min_value(), i8::max_value()-1);
        // right branch
        universe.set_alive(i8::min_value(), i8::min_value()+2);
        universe.set_alive(i8::min_value(), i8::min_value()+3);
        universe.set_alive(i8::min_value(), i8::min_value()+4);
        // top branch
        universe.set_alive(i8::max_value()-3, i8::min_value());
        universe.set_alive(i8::max_value()-2, i8::min_value());
        universe.set_alive(i8::max_value()-1, i8::min_value());
        // bottom branch
        universe.set_alive(i8::min_value()+2, i8::min_value());
        universe.set_alive(i8::min_value()+3, i8::min_value());
        universe.set_alive(i8::min_value()+4, i8::min_value());
        // ....#....
        // ....#....
        // ....#....
        // ...x.....
        // ###.y.###
        // .........
        // ....#....
        // ....#....
        // ....#....
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Alive, Status::Alive, Status::Alive, ],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
        ];
        assert_eq!(universe.window(i8::min_value(), i8::min_value(),i8::min_value()+5, i8::min_value()+5,), expected_universe);

        let universe = universe.next_gen();
        // .........
        // ...###...
        // .........
        // .#.x...#.
        // .#..y..#.
        // .#.....#.
        // .........
        // ...###...
        // .........
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Alive, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Alive, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Alive, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
        ];
        assert_eq!(universe.window(i8::min_value(), i8::min_value(),i8::min_value()+5, i8::min_value()+5,), expected_universe);

        let universe = universe.next_gen();
        // ....#....
        // ....#....
        // ....#....
        // ...x.....
        // ###.y.###
        // .........
        // ....#....
        // ....#....
        // ....#....
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Alive, Status::Alive, Status::Alive, ],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
            vec![Status::Alive, Status::Dead, Status::Dead, Status::Dead, Status::Dead, ],
        ];
        assert_eq!(universe.window(i8::min_value(), i8::min_value(),i8::min_value()+5, i8::min_value()+5,), expected_universe);
    }
}
