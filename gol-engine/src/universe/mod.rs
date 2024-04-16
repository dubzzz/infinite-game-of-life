mod sparse_grid;
mod tests;
mod window;

use sparse_grid::SparseGrid;
use window::Window;

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

    pub fn from(universe: &Universe) -> Universe {
        Universe {
            inhabitants: SparseGrid::from(&universe.inhabitants),
        }
    }

    pub fn set_alive(&mut self, row_index: i8, column_index: i8) -> () {
        self.inhabitants.append_value(row_index, column_index)
    }

    fn is_alive(&self, row_index: i8, column_index: i8) -> bool {
        self.inhabitants.has_value(row_index, column_index)
    }

    pub fn next_gen(&self) -> Self {
        let mut new_inhabitants = SparseGrid::new();
        let mut already_scanned = SparseGrid::new();

        let possible_inhabitants = self
            .inhabitants
            .iter()
            .flat_map(|inhabitant| Window::neighborhood(inhabitant.0, inhabitant.1));
        for (row_index, column_index) in possible_inhabitants {
            if already_scanned.has_value(row_index, column_index) {
                continue;
            }
            already_scanned.append_value(row_index, column_index);

            let current_alive = self.is_alive(row_index, column_index);
            let num_alives = Window::neighborhood(row_index, column_index)
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
        Window::scan(
            row_index_start,
            column_index_start,
            row_index_end,
            column_index_end,
            self,
            |data, row_index, column_index| {
                if data.is_alive(row_index, column_index) {
                    Status::Alive
                } else {
                    Status::Dead
                }
            },
        )
    }
}
