/// A 2D grid of cells with toroidal wrapping (edges connect to opposite sides).
///
/// The grid is the fundamental spatial structure for all 2D cellular automata.
/// It stores cell states and provides neighbor queries that rules need to
/// compute the next generation.
#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

/// The state of a single cell.
///
/// Kept deliberately simple — for Life-like automata, a cell is either
/// alive or dead. More exotic automata (Wireworld, multi-state rules)
/// would extend this enum.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Dead,
    Alive,
}

impl Grid {
    /// Creates a new grid with all cells dead.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![CellState::Dead; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the state of the cell at (x, y).
    ///
    /// Coordinates wrap toroidally — going off the right edge puts you
    /// on the left, going off the bottom puts you on top.
    pub fn get(&self, x: isize, y: isize) -> CellState {
        let x = x.rem_euclid(self.width as isize) as usize;
        let y = y.rem_euclid(self.height as isize) as usize;
        self.cells[y * self.width + x]
    }

    /// Sets the state of the cell at (x, y).
    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = state;
        }
    }

    /// Counts the number of alive neighbors in the Moore neighborhood
    /// (the 8 surrounding cells).
    pub fn alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let x = x as isize;
        let y = y as isize;
        let mut count = 0u8;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get(x + dx, y + dy) == CellState::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    /// Returns total number of alive cells.
    pub fn population(&self) -> usize {
        self.cells.iter().filter(|c| **c == CellState::Alive).count()
    }
}
