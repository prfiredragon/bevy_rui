use bevy::prelude::*;

#[derive(Default)]
pub struct RuiClipboard {
    pub clipboard: Option<arboard::Clipboard>,
    pub local_buffer: String,
}

impl RuiClipboard {
    pub fn set_text(&mut self, text: String) {
        self.local_buffer = text.clone();
        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => error!("No se pudo conectar al portapapeles del SO: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            if let Err(e) = clip.set_text(text) {
                error!("Fallo al copiar al OS: {}", e);
            }
        }
    }

    pub fn get_text(&mut self) -> Option<String> {
        if self.clipboard.is_none() {
            match arboard::Clipboard::new() {
                Ok(c) => self.clipboard = Some(c),
                Err(e) => error!("No se pudo conectar al portapapeles del SO: {}", e),
            }
        }
        if let Some(clip) = &mut self.clipboard {
            match clip.get_text() {
                Ok(text) => return Some(text),
                Err(e) => warn!("Fallo al leer OS (usando buffer interno): {}", e),
            }
        }
        Some(self.local_buffer.clone())
    }
}