mod sparse_grid;
use sparse_grid::SparseGrid;

#[derive(Clone)]
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

        for (row_index, column_index) in self.inhabitants.iter() {
            for (new_row_index, new_column_index) in Self::neighborhood(row_index, column_index) {
                if !already_scanned.has_value(new_row_index, new_column_index) {
                    already_scanned.append_value(new_row_index, new_column_index);
                    let current_alive = self.is_alive(new_row_index, new_column_index);
                    let num_alives = Self::neighborhood(row_index, column_index)
                        .iter()
                        .filter(|(r, c)| self.is_alive(*r, *c))
                        .count();
                    if current_alive && (num_alives == 3 || num_alives == 4) {
                        new_inhabitants.append_value(new_row_index, new_column_index);
                    } else if !current_alive && num_alives == 3 {
                        new_inhabitants.append_value(new_row_index, new_column_index);
                    }
                }
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
