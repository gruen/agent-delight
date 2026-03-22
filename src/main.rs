mod grid;
mod patterns;
mod renderer;
mod rule;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

use grid::Grid;
use patterns::{all_patterns, seed_pattern};
use renderer::Renderer;
use rule::{LifeLike, Rule};

/// Simulation state — everything needed to describe the current run.
struct Simulation {
    grid: Grid,
    generation: u64,
    paused: bool,
    rule: Box<dyn Rule>,
    pattern_index: usize,
    tick_ms: u64,
}

impl Simulation {
    fn new(width: usize, height: usize) -> Self {
        let rule = Box::new(LifeLike::conway());
        let mut grid = Grid::new(width, height);
        let patterns = all_patterns();
        seed_pattern(&mut grid, &patterns[0]);

        Self {
            grid,
            generation: 0,
            paused: true,
            rule,
            pattern_index: 0,
            tick_ms: 80,
        }
    }

    fn step(&mut self) {
        self.grid = self.rule.step(&self.grid);
        self.generation += 1;
    }

    fn reset(&mut self) {
        let patterns = all_patterns();
        self.grid = Grid::new(self.grid.width(), self.grid.height());
        seed_pattern(&mut self.grid, &patterns[self.pattern_index]);
        self.generation = 0;
        self.paused = true;
    }

    fn next_pattern(&mut self) {
        let patterns = all_patterns();
        self.pattern_index = (self.pattern_index + 1) % patterns.len();
        self.reset();
    }
}

fn main() -> io::Result<()> {
    // Determine grid size from terminal dimensions.
    let (term_w, term_h) = terminal::size()?;
    // Each cell is 2 chars wide; leave margin for border and status bar.
    let grid_w = ((term_w.saturating_sub(4)) / 2) as usize;
    let grid_h = (term_h.saturating_sub(6)) as usize;

    // Minimum viable size.
    let grid_w = grid_w.max(20);
    let grid_h = grid_h.max(10);

    let mut sim = Simulation::new(grid_w, grid_h);
    let renderer = Renderer::new();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    renderer.clear(&mut stdout)?;

    loop {
        let frame_start = Instant::now();

        // Handle input.
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => sim.paused = !sim.paused,
                        KeyCode::Char('n') => {
                            sim.step();
                        }
                        KeyCode::Char('r') => {
                            sim.reset();
                            renderer.clear(&mut stdout)?;
                        }
                        KeyCode::Char('p') => {
                            sim.next_pattern();
                            renderer.clear(&mut stdout)?;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            sim.tick_ms = sim.tick_ms.saturating_sub(10).max(10);
                        }
                        KeyCode::Char('-') => {
                            sim.tick_ms = (sim.tick_ms + 10).min(500);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Advance simulation if running.
        if !sim.paused {
            sim.step();
        }

        // Render.
        renderer.draw(
            &mut stdout,
            &sim.grid,
            sim.generation,
            sim.paused,
            sim.rule.name(),
        )?;

        // Frame rate control.
        let elapsed = frame_start.elapsed();
        let target = Duration::from_millis(sim.tick_ms);
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }

    // Cleanup.
    renderer.cleanup(&mut stdout)?;
    terminal::disable_raw_mode()?;
    renderer.clear(&mut stdout)?;

    println!("lattice — {} generations simulated.", sim.generation);
    Ok(())
}
