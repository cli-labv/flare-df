//! FLARE-DF - Banner animado
//! ==========================
//!
//! Muestra el banner de bienvenida con animación.

use colored::Colorize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::config::{APP_VERSION, APP_URL};

/// Arte ASCII del banner con llamas
const FLAME_ART: &[(&str, &str)] = &[
    ("                          🔥", "bright_red"),
    ("                        🔥🔥🔥", "red"),
    ("                      🔥🔥🔥🔥🔥", "yellow"),
];

/// Logo principal
const LOGO_LINES: &[(&str, &str)] = &[
    ("", ""),
    ("    ███████╗██╗      █████╗ ██████╗ ███████╗      ██████╗ ███████╗", "bright_red"),
    ("    ██╔════╝██║     ██╔══██╗██╔══██╗██╔════╝      ██╔══██╗██╔════╝", "red"),
    ("    █████╗  ██║     ███████║██████╔╝█████╗  █████╗██║  ██║█████╗  ", "yellow"),
    ("    ██╔══╝  ██║     ██╔══██║██╔══██╗██╔══╝  ╚════╝██║  ██║██╔══╝  ", "bright_yellow"),
    ("    ██║     ███████╗██║  ██║██║  ██║███████╗      ██████╔╝██║     ", "white"),
    ("    ╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝      ╚═════╝ ╚═╝     ", "bright_white"),
    ("", ""),
];

/// Tagline
const TAGLINE_LINES: &[(&str, &str)] = &[
    ("    ╔══════════════════════════════════════════════════════════════════════╗", "cyan"),
    ("    ║        🔥 Comprime PDFs al instante sin perder calidad 🔥             ║", "bright_cyan"),
    ("    ║           ⚡ Rápido • 🎯 Lossless • 🦀 Powered by Rust                ║", "cyan"),
    ("    ╚══════════════════════════════════════════════════════════════════════╝", "cyan"),
];

/// Aplica color a un texto
fn colorize(text: &str, color: &str) -> String {
    match color {
        "bright_red" => text.bright_red().to_string(),
        "red" => text.red().to_string(),
        "yellow" => text.yellow().to_string(),
        "bright_yellow" => text.bright_yellow().to_string(),
        "white" => text.white().to_string(),
        "bright_white" => text.bright_white().to_string(),
        "cyan" => text.cyan().to_string(),
        "bright_cyan" => text.bright_cyan().to_string(),
        "dim" => text.dimmed().to_string(),
        _ => text.to_string(),
    }
}

/// Imprime texto con animación de escritura
fn animate_typing(text: &str, color: &str, char_delay_ms: u64) {
    let colored_text = colorize(text, color);
    for ch in colored_text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        if !ch.is_whitespace() {
            thread::sleep(Duration::from_micros(char_delay_ms * 100));
        }
    }
    println!();
}

/// Imprime una línea con delay
fn animate_line(text: &str, color: &str, line_delay_ms: u64) {
    println!("{}", colorize(text, color));
    thread::sleep(Duration::from_millis(line_delay_ms));
}

/// Muestra el banner con animación progresiva
pub fn show_animated_banner() {
    // Limpiar pantalla
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
    
    // Mostrar llamas
    for (text, color) in FLAME_ART {
        animate_line(text, color, 50);
    }
    
    // Mostrar logo con efecto typing
    for (text, color) in LOGO_LINES {
        if text.is_empty() {
            println!();
        } else {
            animate_typing(text, color, 8);
        }
        thread::sleep(Duration::from_millis(20));
    }
    
    // Pausa dramática
    thread::sleep(Duration::from_millis(100));
    
    // Mostrar tagline
    for (text, color) in TAGLINE_LINES {
        animate_line(text, color, 40);
    }
    
    // Información de versión
    println!();
    println!("{}", colorize(
        &format!("                         v{} │ {}", APP_VERSION, APP_URL),
        "dim"
    ));
    println!();
    
    // Separador
    let separator = "    ".to_owned() + &"═".repeat(72);
    println!("{}", colorize(&separator, "dim"));
    println!();
}

/// Muestra el banner sin animación
pub fn show_simple_banner() {
    print!("\x1B[2J\x1B[H");
    
    for (text, color) in FLAME_ART {
        println!("{}", colorize(text, color));
    }
    
    for (text, color) in LOGO_LINES {
        println!("{}", colorize(text, color));
    }
    
    for (text, color) in TAGLINE_LINES {
        println!("{}", colorize(text, color));
    }
    
    println!();
    println!("{}", colorize(
        &format!("                         v{} │ {}", APP_VERSION, APP_URL),
        "dim"
    ));
    println!();
}

/// Muestra mensaje de despedida
pub fn show_goodbye() {
    println!();
    println!("{}", "    🔥 ¡Hasta pronto! Gracias por usar FLARE-DF 🔥".cyan());
    println!();
}
