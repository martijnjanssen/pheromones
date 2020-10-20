mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Ground = 0,
    Wall = 1,
    Start = 2,
    End = 3,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    ants: Vec<Ant>,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Ground => Cell::Wall,
            Cell::Wall => Cell::Ground,
            e => e,
        };
    }

    fn start(&mut self) {
        *self = Cell::Start
    }

    fn end(&mut self) {
        *self = Cell::End
    }
}

pub struct Ant {}
impl Ant {}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let ants_num = 100;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Ground
                } else {
                    Cell::Wall
                }
            })
            .collect();

        let ants = (0..ants_num).map(|i| Ant {}).collect();

        Universe {
            width,
            height,
            cells,
            ants,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn remove_special(&mut self, special: Cell) {
        self.cells = self
            .cells
            .iter()
            .map(|c| {
                if *c == special {
                    return Cell::Ground;
                }
                return *c;
            })
            .collect();
    }

    pub fn toggle_wall(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn set_start(&mut self, row: u32, column: u32) {
        self.remove_special(Cell::Start);
        let idx = self.get_index(row, column);
        self.cells[idx].start();
    }

    pub fn set_end(&mut self, row: u32, column: u32) {
        self.remove_special(Cell::End);
        let idx = self.get_index(row, column);
        self.cells[idx].end();
    }

    fn live_neightbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();

        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         let cell = self.cells[idx];
        //         let live_neighbors = self.live_neightbor_count(row, col);

        //         let next_cell = match (cell, live_neighbors) {
        //             // Rule 1: Any live cell with fewer than two live neighbours
        //             // dies, as if caused by underpopulation.
        //             (Cell::Alive, x) if x < 2 => Cell::Dead,
        //             // Rule 2: Any live cell with two or three live neighbours
        //             // lives on to the next generation.
        //             (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
        //             // Rule 3: Any live cell with more than three live
        //             // neighbours dies, as if by overpopulation.
        //             (Cell::Alive, x) if x > 3 => Cell::Dead,
        //             // Rule 4: Any dead cell with exactly three live neighbours
        //             // becomes a live cell, as if by reproduction.
        //             (Cell::Dead, 3) => Cell::Alive,
        //             // All other cells remain in the same state.
        //             (otherwise, _) => otherwise,
        //         };

        //         next[idx] = next_cell;
        //     }
        // }

        // self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
