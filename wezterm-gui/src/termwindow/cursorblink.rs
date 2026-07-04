use std::time::Instant;

/// Tracks when the cursor blink phase was last reset, so that the
/// cursor renders solid (and restarts its blink cycle) in response
/// to user interaction: keyboard input and window focus.
///
/// The phase is deliberately NOT reset when the application moves
/// the cursor: full-screen programs reposition the cursor on every
/// repaint, and anchoring the blink phase to cursor movement caused
/// the cursor to shimmer at the program's repaint rate instead of
/// blinking at the configured cursor_blink_rate.
#[derive(Clone)]
pub struct CursorBlinkPhase {
    reset_at: Instant,
}

impl CursorBlinkPhase {
    pub fn new() -> Self {
        CursorBlinkPhase {
            reset_at: Instant::now(),
        }
    }

    /// Restart the blink cycle; the cursor renders solid from now.
    pub fn reset(&mut self) {
        self.reset_at = Instant::now();
    }

    /// When was the blink phase last reset?
    pub fn last_reset(&self) -> Instant {
        self.reset_at
    }
}
