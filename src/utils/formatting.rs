//! FLARE-PDF - Utilidades de formato
//! ==================================
//!
//! Funciones de ayuda para formateo de datos.

use bytesize::ByteSize;

/// Calcula el ancho visual de una cadena considerando emojis
/// Los emojis son más anchos en la terminal (cuentan como 2 caracteres)
pub fn visual_width(s: &str) -> usize {
    let mut width = 0;
    for ch in s.chars() {
        // Los emojis y caracteres especiales ocupan 2 espacios
        if is_wide_char(ch) {
            width += 2;
        } else if ch == '\n' {
            // No contar saltos de línea
            continue;
        } else {
            width += 1;
        }
    }
    width
}

/// Determina si un carácter es ancho (emoji u otro)
fn is_wide_char(ch: char) -> bool {
    // Rango de emojis y caracteres especiales
    let code = ch as u32;
    (code >= 0x1F300 && code <= 0x1F9FF) || // Emojis
    (code >= 0x2600 && code <= 0x27BF) ||   // Símbolos variados
    (code >= 0x1F000 && code <= 0x1F02F)    // Símbolos decorativos
}

/// Crea padding dinámico para alineación
pub fn pad_right(s: &str, width: usize) -> String {
    let visual_len = visual_width(s);
    if visual_len >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - visual_len))
    }
}

/// Crea padding izquierdo
pub fn pad_left(s: &str, width: usize) -> String {
    let visual_len = visual_width(s);
    if visual_len >= width {
        s.to_string()
    } else {
        format!("{}{}", " ".repeat(width - visual_len), s)
    }
}

/// Crea padding en ambos lados (centrado)
pub fn pad_center(s: &str, width: usize) -> String {
    let visual_len = visual_width(s);
    if visual_len >= width {
        s.to_string()
    } else {
        let total_padding = width - visual_len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        format!("{}{}{}", " ".repeat(left_padding), s, " ".repeat(right_padding))
    }
}

/// Formatea bytes a formato legible
pub fn format_bytes(bytes: u64) -> String {
    ByteSize(bytes).to_string_as(true)
}

/// Trunca texto a una longitud máxima
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.saturating_sub(3)])
    }
}

/// Formatea un porcentaje
pub fn format_percent(value: f64) -> String {
    format!("{:.1}%", value)
}

/// Formatea duración en segundos a formato legible
pub fn format_duration(seconds: f64) -> String {
    if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else if seconds < 3600.0 {
        let mins = (seconds / 60.0).floor();
        let secs = seconds % 60.0;
        format!("{}m {:.0}s", mins, secs)
    } else {
        let hours = (seconds / 3600.0).floor();
        let mins = ((seconds % 3600.0) / 60.0).floor();
        format!("{}h {}m", hours, mins)
    }
}
