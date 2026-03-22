use std::io::{self, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};

use crate::grid::{CellState, Grid};

/// Renders a grid to the terminal using crossterm.
///
/// Uses block characters for a dense, clean look. Alive cells render as
/// bright filled blocks; dead cells are spaces. The renderer is stateless —
/// it redraws the full grid each frame, relying on the terminal's own
/// buffering for performance.
pub struct Renderer {
    /// Offset from top-left of terminal to start drawing.
    offset_x: u16,
    offset_y: u16,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            offset_x: 1,
            offset_y: 1,
        }
    }

    /// Draw the grid to the terminal buffer. Call flush() after.
    pub fn draw(
        &self,
        stdout: &mut io::Stdout,
        grid: &Grid,
        generation: u64,
        paused: bool,
        rule_name: &str,
        pattern_name: &str,
    ) -> io::Result<()> {
        stdout.queue(cursor::Hide)?;

        for y in 0..grid.height() {
            stdout.queue(cursor::MoveTo(
                self.offset_x,
                self.offset_y + y as u16,
            ))?;
            for x in 0..grid.width() {
                match grid.get(x as isize, y as isize) {
                    CellState::Alive => {
                        stdout.queue(style::PrintStyledContent("██".white()))?;
                    }
                    CellState::Dead => {
                        stdout.queue(style::PrintStyledContent("  ".dark_grey()))?;
                    }
                }
            }
        }

        // Status bar
        let status_y = self.offset_y + grid.height() as u16 + 1;
        stdout.queue(cursor::MoveTo(self.offset_x, status_y))?;
        let state_label = if paused { "PAUSED" } else { "RUNNING" };
        let status = format!(
            " {} | {} | Gen {} | Pop {} | {} ",
            rule_name,
            pattern_name,
            generation,
            grid.population(),
            state_label,
        );
        stdout.queue(style::PrintStyledContent(status.on_dark_blue().white()))?;

        // Controls
        stdout.queue(cursor::MoveTo(self.offset_x, status_y + 1))?;
        stdout.queue(style::PrintStyledContent(
            " [Space] pause  [N] step  [R] reset  [P] pattern  [Tab] rule  [+/-] speed  [Q] quit "
                .dark_grey(),
        ))?;

        stdout.flush()?;
        Ok(())
    }

    /// Clear the screen.
    pub fn clear(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.flush()?;
        Ok(())
    }

    /// Show the cursor again (call on exit).
    pub fn cleanup(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(cursor::Show)?;
        stdout.flush()?;
        Ok(())
    }
}
