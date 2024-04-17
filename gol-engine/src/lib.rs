mod serializer;
mod universe;

extern crate wasm_bindgen;

use universe::Universe;
use wasm_bindgen::prelude::*;

use crate::{serializer::Serializer, universe::Status};

#[wasm_bindgen]
pub struct UniverseWasm {
    universe: Universe,
}

#[wasm_bindgen]
impl UniverseWasm {
    #[wasm_bindgen]
    pub fn new() -> UniverseWasm {
        UniverseWasm {
            universe: Universe::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add(&mut self, row_index: isize, column_index: isize) -> UniverseWasm {
        let mut new_universe = Universe::from(&self.universe);
        new_universe.set_alive(row_index, column_index);
        UniverseWasm {
            universe: new_universe,
        }
    }

    #[wasm_bindgen]
    pub fn add_many(&mut self, row_indices: &[isize], column_indices: &[isize]) -> UniverseWasm {
        let mut new_universe = Universe::from(&self.universe);
        for index in 0..row_indices.len() {
            let row_index = row_indices[index];
            let column_index = column_indices[index];
            new_universe.set_alive(row_index, column_index);
        }
        UniverseWasm {
            universe: new_universe,
        }
    }

    #[wasm_bindgen]
    pub fn next(&self) -> UniverseWasm {
        UniverseWasm {
            universe: self.universe.next_gen(),
        }
    }

    #[wasm_bindgen]
    pub fn window(
        &self,
        row_index_start: isize,
        column_index_start: isize,
        row_index_end: isize,
        column_index_end: isize,
    ) -> String {
        let grid = self.universe.window(
            row_index_start,
            column_index_start,
            row_index_end,
            column_index_end,
        );
        Serializer::serialize(&grid, |cell| match cell {
            Status::Dead => " ",
            Status::Alive => "*",
        })
    }
}
