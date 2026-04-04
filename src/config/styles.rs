//! FLARE-PDF - Estilos de progreso
//! ================================
//!
//! Define los estilos disponibles para indicatif.

use crate::models::ProgressStyle;

/// Estilos de spinner disponibles
pub const SPINNER_STYLES: &[(&str, &str)] = &[
    ("flare", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    ("dots", "⣾⣽⣻⢿⡿⣟⣯⣷"),
    ("arrows", "←↖↑↗→↘↓↙"),
    ("bounce", "⠁⠂⠄⠂"),
    ("circle", "◐◓◑◒"),
    ("square", "◰◳◲◱"),
    ("pulse", "█▓▒░▒▓"),
    ("fire", "🔥💥✨🔥"),
];

/// Estilos de barra disponibles
pub const BAR_STYLES: &[(&str, &str)] = &[
    ("smooth", "█▓▒░"),
    ("classic", "=>-"),
    ("blocks", "█▉▊▋▌▍▎▏"),
    ("arrows", "▸▹"),
    ("dots", "●○"),
    ("flames", "🔥▓▒░"),
];

/// Presets de estilos completos
pub fn style_presets() -> Vec<(&'static str, ProgressStyle)> {
    vec![
        ("🔥 Flare (radioactive)", ProgressStyle {
            spinner: "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            bar_chars: "█▓▒░",
            name: "Flare 🔥",
        }),
        ("⚡ Rápido (dots)", ProgressStyle {
            spinner: "⣾⣽⣻⢿⡿⣟⣯⣷",
            bar_chars: "=>-",
            name: "Rápido",
        }),
        ("🌊 Ondas (bounce)", ProgressStyle {
            spinner: "⠁⠂⠄⠂",
            bar_chars: "█▉▊▋▌▍▎▏",
            name: "Ondas",
        }),
        ("🎯 Flechas (arrows)", ProgressStyle {
            spinner: "←↖↑↗→↘↓↙",
            bar_chars: "▸▹",
            name: "Flechas",
        }),
        ("🔵 Círculos (circle)", ProgressStyle {
            spinner: "◐◓◑◒",
            bar_chars: "●○",
            name: "Círculos",
        }),
        ("🎃 Pulso (pulse)", ProgressStyle {
            spinner: "█▓▒░▒▓",
            bar_chars: "█▓▒░",
            name: "Pulso",
        }),
        ("✨ Fuego (fire)", ProgressStyle {
            spinner: "🔥💥✨🔥",
            bar_chars: "🔥▓▒░",
            name: "Fuego",
        }),
    ]
}

/// Obtiene el estilo por defecto
pub fn default_style() -> ProgressStyle {
    ProgressStyle::default()
}
