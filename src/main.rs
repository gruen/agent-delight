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
use patterns::{all_patterns, seed_pattern, seed_random};
use renderer::Renderer;
use rule::{all_rules, Rule};

/// Simulation state — everything needed to describe the current run.
struct Simulation {
    grid: Grid,
    width: usize,
    height: usize,
    generation: u64,
    paused: bool,
    rules: Vec<Box<dyn Rule>>,
    rule_index: usize,
    pattern_index: usize,
    pattern_name: String,
    tick_ms: u64,
}

impl Simulation {
    fn new(width: usize, height: usize) -> Self {
        let rules: Vec<Box<dyn Rule>> = all_rules()
            .into_iter()
            .map(|r| Box::new(r) as Box<dyn Rule>)
            .collect();
        let mut grid = Grid::new(width, height);
        let patterns = all_patterns();
        seed_pattern(&mut grid, &patterns[0]);
        let pattern_name = patterns[0].name.to_string();

        Self {
            grid,
            width,
            height,
            generation: 0,
            paused: true,
            rules,
            rule_index: 0,
            pattern_index: 0,
            pattern_name,
            tick_ms: 80,
        }
    }

    fn rule(&self) -> &dyn Rule {
        self.rules[self.rule_index].as_ref()
    }

    fn step(&mut self) {
        let rule = &self.rules[self.rule_index];
        self.grid = rule.step(&self.grid);
        self.generation += 1;
    }

    fn reset(&mut self) {
        self.grid = Grid::new(self.width, self.height);
        let patterns = all_patterns();
        // pattern_index within range = named pattern, out of range = random
        if self.pattern_index < patterns.len() {
            seed_pattern(&mut self.grid, &patterns[self.pattern_index]);
            self.pattern_name = patterns[self.pattern_index].name.to_string();
        } else {
            seed_random(&mut self.grid, 0.3);
            self.pattern_name = "Random".to_string();
        }
        self.generation = 0;
        self.paused = true;
    }

    fn next_pattern(&mut self) {
        let patterns = all_patterns();
        // patterns.len() entries + 1 for "Random"
        self.pattern_index = (self.pattern_index + 1) % (patterns.len() + 1);
        self.reset();
    }

    fn next_rule(&mut self) {
        self.rule_index = (self.rule_index + 1) % self.rules.len();
        self.reset();
    }
}

fn main() -> io::Result<()> {
    // Determine grid size from terminal dimensions.
    let (term_w, term_h) = terminal::size()?;
    let grid_w = ((term_w.saturating_sub(4)) / 2) as usize;
    let grid_h = (term_h.saturating_sub(6)) as usize;
    let grid_w = grid_w.max(20);
    let grid_h = grid_h.max(10);

    let mut sim = Simulation::new(grid_w, grid_h);
    let renderer = Renderer::new();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    renderer.clear(&mut stdout)?;

    loop {
        let frame_start = Instant::now();

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => sim.paused = !sim.paused,
                        KeyCode::Char('n') => sim.step(),
                        KeyCode::Char('r') => {
                            sim.reset();
                            renderer.clear(&mut stdout)?;
                        }
                        KeyCode::Char('p') => {
                            sim.next_pattern();
                            renderer.clear(&mut stdout)?;
                        }
                        KeyCode::Tab => {
                            sim.next_rule();
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

        if !sim.paused {
            sim.step();
        }

        renderer.draw(
            &mut stdout,
            &sim.grid,
            sim.generation,
            sim.paused,
            sim.rule().name(),
            &sim.pattern_name,
        )?;

        let elapsed = frame_start.elapsed();
        let target = Duration::from_millis(sim.tick_ms);
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }

    renderer.cleanup(&mut stdout)?;
    terminal::disable_raw_mode()?;
    renderer.clear(&mut stdout)?;

    println!("lattice — {} generations simulated.", sim.generation);
    Ok(())
}
