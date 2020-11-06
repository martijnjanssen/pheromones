mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MAX_ANT_STEPS: u32 = 200;

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

    start_row: u32,
    start_column: u32,
    end_row: u32,
    end_column: u32,

    cells: Vec<Cell>,
    pheromones: Vec<u32>,
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

#[derive(Clone, Debug)]
pub struct Ant {
    alive: bool,
    row: u32,
    column: u32,
    route: Vec<(u32, u32)>,
}

impl Ant {
    fn check_dead(&mut self) -> bool {
        if self.route.len() as u32 > MAX_ANT_STEPS {
            self.alive = false;
            return true;
        }
        return false;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let start_row: u32 = 0;
        let start_column: u32 = 0;
        let end_row: u32 = width;
        let end_column: u32 = height;

        let cells = (0..width * height)
            .map(|i| {
                if i == 0 {
                    Cell::Start
                } else if i == width * height - 1 {
                    Cell::End
                } else if i % 2 == 0 || i % 7 == 0 {
                    Cell::Ground
                } else {
                    Cell::Wall
                }
            })
            .collect();
        let pheromones = Vec::new();

        let ants_num = 100;
        let ants = (0..ants_num)
            .map(|i| Ant {
                alive: true,
                row: 0,
                column: 0,
                route: Vec::with_capacity(MAX_ANT_STEPS as usize),
            })
            .collect();

        Universe {
            width,
            height,

            start_row,
            start_column,
            end_row,
            end_column,

            cells,
            pheromones,
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

        self.start_row = row;
        self.start_column = column;
    }

    pub fn set_end(&mut self, row: u32, column: u32) {
        self.remove_special(Cell::End);
        let idx = self.get_index(row, column);
        self.cells[idx].end();

        self.end_row = row;
        self.end_column = column;
    }

    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }

    //     count
    // }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        let mut next_pheromones = self.pheromones.clone();
        let mut next_ants = self.ants.clone();

        for mut ant in next_ants {
            // TODO: Get ant position
            let row = ant.row;
            let column = ant.column;

            // TODO: Check ant total steps, if too high, die
            if ant.check_dead() {
                ant.alive = false;
                continue;
            }

            // TODO: Check directions for out of bounds

            // TODO: Weighted random choice based on pheromones

            // TODO: Add coordinates to list
        }

        let alive_ants: Vec<Ant> = next_ants
            .clone()
            .drain(..)
            .filter(|mut ant| ant.check_dead())
            .collect();

        for i in 0..next_ants.len() {
            if !next_ants[i].alive {
                next_ants.remove(i);
            }
        }

        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         let cell = self.cells[idx];
        //         let live_neighbors = self.live_neighbor_count(row, col);

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

        self.cells = next;
        self.pheromones = next_pheromones;
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
