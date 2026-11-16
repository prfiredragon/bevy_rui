//! OS clipboard integration with Wayland (`wl-clipboard`), `arboard`, and in-memory fallback (`non-wasm32`).

use bevy::prelude::*;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

/// Non-send resource providing clipboard copy/paste via native OS clipboard (`wl-clipboard` on Wayland + `arboard`),
/// falling back to `local_buffer` if unavailable.
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

        #[cfg(target_os = "linux")]
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            if let Ok(mut child) = Command::new("wl-copy")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
            }
        }

        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => warn!("Could not connect to arboard OS clipboard: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            if let Err(e) = clip.set_text(text) {
                warn!("Failed to copy to arboard clipboard: {}", e);
            }
        }
    }

    /// Reads text from the OS clipboard, falling back to `local_buffer` on failure.
    pub fn get_text(&mut self) -> Option<String> {
        #[cfg(target_os = "linux")]
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            if let Ok(output) = Command::new("wl-paste")
                .arg("--no-newline")
                .stderr(Stdio::null())
                .output()
            {
                if output.status.success() {
                    if let Ok(s) = String::from_utf8(output.stdout) {
                        if !s.is_empty() {
                            self.local_buffer = s.clone();
                            return Some(s);
                        }
                    }
                }
            }
            // Fallback to Wayland primary selection (e.g. text highlighted in terminal)
            if let Ok(output) = Command::new("wl-paste")
                .args(["--primary", "--no-newline"])
                .stderr(Stdio::null())
                .output()
            {
                if output.status.success() {
                    if let Ok(s) = String::from_utf8(output.stdout) {
                        if !s.is_empty() {
                            self.local_buffer = s.clone();
                            return Some(s);
                        }
                    }
                }
            }
        }

        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => warn!("Could not connect to arboard OS clipboard: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            match clip.get_text() {
                Ok(text) if !text.is_empty() => {
                    self.local_buffer = text.clone();
                    return Some(text);
                }
                Ok(_) => {}
                Err(e) => warn!("Failed to read OS clipboard (using local buffer): {}", e),
            }
        }
        Some(self.local_buffer.clone())
    }
}