//! OS clipboard integration with an in-memory fallback buffer (`non-wasm32`).

use bevy::prelude::*;

/// Non-send resource providing clipboard copy/paste via `arboard`, falling back to `local_buffer` if unavailable.
#[derive(Default)]
pub struct RuiClipboard {
    /// Native OS clipboard handle (lazily initialized).
    pub clipboard: Option<arboard::Clipboard>,
    /// Fallback in-memory clipboard string buffer.
    pub local_buffer: String,
}

impl RuiClipboard {
    /// Copies `text` into both the local fallback buffer and the OS clipboard.
    pub fn set_text(&mut self, text: String) {
        self.local_buffer = text.clone();
        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => error!("Could not connect to OS clipboard: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            if let Err(e) = clip.set_text(text) {
                error!("Failed to copy to OS clipboard: {}", e);
            }
        }
    }

    /// Reads text from the OS clipboard, falling back to `local_buffer` on failure.
    pub fn get_text(&mut self) -> Option<String> {
        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => error!("Could not connect to OS clipboard: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            match clip.get_text() {
                Ok(text) => return Some(text),
                Err(e) => warn!("Failed to read OS clipboard (using local buffer): {}", e),
            }
        }
        Some(self.local_buffer.clone())
    }
}