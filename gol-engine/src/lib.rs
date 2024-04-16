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
    pub fn new() -> UniverseWasm {
        UniverseWasm {
            universe: Universe::new(),
        }
    }

    pub fn add(&mut self, row_index: i8, column_index: i8) -> UniverseWasm {
        let mut new_universe = Universe::from(&self.universe);
        new_universe.set_alive(row_index, column_index);
        UniverseWasm {
            universe: new_universe,
        }
    }

    pub fn next(&self) -> UniverseWasm {
        UniverseWasm {
            universe: self.universe.next_gen(),
        }
    }

    pub fn window(
        &self,
        row_index_start: i8,
        column_index_start: i8,
        row_index_end: i8,
        column_index_end: i8,
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
