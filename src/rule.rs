use crate::grid::Grid;

/// The core abstraction: a rule that computes the next generation of a grid.
///
/// Any cellular automaton that operates on a 2D grid of alive/dead cells
/// can implement this trait. The rule examines the current grid and produces
/// a new one — grids are never mutated in place during a step, which keeps
/// the semantics clean (all cells update simultaneously).
pub trait Rule: Send {
    /// Human-readable name for display.
    fn name(&self) -> &str;

    /// Compute the next generation from the current grid.
    fn step(&self, grid: &Grid) -> Grid;
}

/// A Life-like rule defined by birth and survival counts.
///
/// Conway's Game of Life is B3/S23: a dead cell with exactly 3 alive neighbors
/// is born; a live cell with 2 or 3 alive neighbors survives. All other cells
/// die or stay dead.
///
/// This generalizes to any combination — HighLife is B36/S23, Day & Night is
/// B3678/S34678, etc.
pub struct LifeLike {
    name: String,
    /// Neighbor counts that cause a dead cell to become alive.
    birth: [bool; 9],
    /// Neighbor counts that let a living cell survive.
    survival: [bool; 9],
}

impl LifeLike {
    /// Creates a Life-like rule from birth and survival count slices.
    ///
    /// # Example
    /// ```
    /// // Conway's Game of Life: B3/S23
    /// let life = LifeLike::new("Conway's Life", &[3], &[2, 3]);
    /// ```
    pub fn new(name: &str, birth: &[u8], survival: &[u8]) -> Self {
        let mut b = [false; 9];
        let mut s = [false; 9];
        for &count in birth {
            if (count as usize) < 9 {
                b[count as usize] = true;
            }
        }
        for &count in survival {
            if (count as usize) < 9 {
                s[count as usize] = true;
            }
        }
        Self {
            name: name.to_string(),
            birth: b,
            survival: s,
        }
    }

    /// Conway's Game of Life — B3/S23.
    pub fn conway() -> Self {
        Self::new("Conway's Life", &[3], &[2, 3])
    }

    /// HighLife — B36/S23. Notable for its replicator pattern.
    pub fn highlife() -> Self {
        Self::new("HighLife", &[3, 6], &[2, 3])
    }

    /// Day & Night — B3678/S34678. Symmetric between alive and dead.
    pub fn day_and_night() -> Self {
        Self::new("Day & Night", &[3, 6, 7, 8], &[3, 4, 6, 7, 8])
    }
}

/// Returns all built-in Life-like rules.
pub fn all_rules() -> Vec<LifeLike> {
    vec![
        LifeLike::conway(),
        LifeLike::highlife(),
        LifeLike::day_and_night(),
        LifeLike::new("Seeds", &[2], &[]),
        LifeLike::new("Diamoeba", &[3, 5, 6, 7, 8], &[5, 6, 7, 8]),
    ]
}

impl Rule for LifeLike {
    fn name(&self) -> &str {
        &self.name
    }

    fn step(&self, grid: &Grid) -> Grid {
        use crate::grid::CellState::{Alive, Dead};

        let mut next = Grid::new(grid.width(), grid.height());
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let neighbors = grid.alive_neighbors(x, y);
                let state = match grid.get(x as isize, y as isize) {
                    Alive => {
                        if self.survival[neighbors as usize] {
                            Alive
                        } else {
                            Dead
                        }
                    }
                    Dead => {
                        if self.birth[neighbors as usize] {
                            Alive
                        } else {
                            Dead
                        }
                    }
                };
                next.set(x, y, state);
            }
        }
        next
    }
}
