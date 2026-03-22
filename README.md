# lattice

A terminal-based cellular automaton explorer, written in Rust.

Built by **Tempo** (an AI) in collaboration with a human. See [DEVLOG.md](DEVLOG.md)
for development notes and [SOCIAL.md](SOCIAL.md) for public posts about the build.

## What it does

Run cellular automata in your terminal. Watch patterns evolve in real time.
Pause, step through generations, switch patterns, adjust speed.

Currently supports Life-like automata (Conway's Game of Life, HighLife,
Day & Night) with built-in patterns including the Glider, R-pentomino,
Acorn, and Gosper Glider Gun.

## Controls

| Key       | Action                    |
|-----------|---------------------------|
| `Space`   | Pause / resume            |
| `N`       | Step one generation       |
| `R`       | Reset to initial state    |
| `P`       | Cycle to next pattern     |
| `+` / `=` | Speed up                  |
| `-`       | Slow down                 |
| `Q` / Esc | Quit                      |

## Building

```
cargo build --release
cargo run --release
```

## Architecture

- **`grid.rs`** — 2D toroidal grid with Moore neighborhood queries
- **`rule.rs`** — `Rule` trait + `LifeLike` generalized rule implementation
- **`patterns.rs`** — Named seed patterns as coordinate offsets
- **`renderer.rs`** — Terminal rendering via crossterm
- **`main.rs`** — Event loop, input handling, simulation state

Key invariant: rules produce new grids, never mutate in place. This matches
the simultaneous-update semantics of cellular automata.

## License

MIT
