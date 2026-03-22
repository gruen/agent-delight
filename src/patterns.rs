use crate::grid::{CellState, Grid};

/// Seed a grid with a known pattern centered in the grid.
///
/// Patterns are defined as coordinate offsets from center. This keeps
/// them readable and easy to add new ones.
pub fn seed_pattern(grid: &mut Grid, pattern: &Pattern) {
    let cx = grid.width() / 2;
    let cy = grid.height() / 2;
    for &(dx, dy) in &pattern.offsets {
        let x = (cx as isize + dx) as usize;
        let y = (cy as isize + dy) as usize;
        grid.set(x, y, CellState::Alive);
    }
}

/// A named pattern defined by coordinate offsets from center.
pub struct Pattern {
    pub name: &'static str,
    pub offsets: Vec<(isize, isize)>,
}

/// The R-pentomino: 5 cells that evolve for 1103 generations before stabilizing.
/// A classic example of how tiny initial conditions produce complex behavior.
pub fn r_pentomino() -> Pattern {
    Pattern {
        name: "R-pentomino",
        offsets: vec![(0, -1), (1, -1), (-1, 0), (0, 0), (0, 1)],
    }
}

/// The Gosper Glider Gun: the first known finite pattern that grows without bound.
/// Emits a new glider every 30 generations.
pub fn glider_gun() -> Pattern {
    Pattern {
        name: "Gosper Glider Gun",
        offsets: vec![
            // Left block
            (-18, 0), (-18, 1), (-17, 0), (-17, 1),
            // Left structure
            (-8, 0), (-8, 1), (-8, 2),
            (-7, -1), (-7, 3),
            (-6, -2), (-6, 4),
            (-5, -2), (-5, 4),
            (-4, 1),
            (-3, -1), (-3, 3),
            (-2, 0), (-2, 1), (-2, 2),
            (-1, 1),
            // Right structure
            (2, 0), (2, -1), (2, -2),
            (3, 0), (3, -1), (3, -2),
            (4, -3), (4, 1),
            (6, -3), (6, -4), (6, 1), (6, 2),
            // Right block
            (16, -1), (16, -2), (17, -1), (17, -2),
        ],
    }
}

/// Acorn: 7 cells that take 5206 generations to stabilize. Produces 633 cells.
pub fn acorn() -> Pattern {
    Pattern {
        name: "Acorn",
        offsets: vec![
            (-3, -1), (-2, 1), (-1, -1), (-1, 0), (1, 0), (2, 0), (3, 0),
        ],
    }
}

/// A simple glider — the smallest moving pattern in Life.
pub fn glider() -> Pattern {
    Pattern {
        name: "Glider",
        offsets: vec![(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)],
    }
}

/// Returns all built-in patterns.
pub fn all_patterns() -> Vec<Pattern> {
    vec![glider(), r_pentomino(), acorn(), glider_gun()]
}
