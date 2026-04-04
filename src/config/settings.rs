//! FLARE-DF - Configuración y constantes
//! ======================================
//!
//! Centraliza toda la configuración de la aplicación.

use std::path::PathBuf;
use std::env;

/// Nombre de la aplicación
pub const APP_NAME: &str = "FLARE-DF";

/// Versión de la aplicación
pub const APP_VERSION: &str = "1.0.0";

/// URL del proyecto
pub const APP_URL: &str = "github.com/flare-df";

/// Extensiones de PDF soportadas
pub const PDF_EXTENSIONS: &[&str] = &[".pdf", ".PDF"];

/// Obtiene el directorio del proyecto
pub fn project_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Obtiene el directorio de entrada
pub fn input_dir() -> PathBuf {
    project_dir().join("input")
}

/// Obtiene el directorio de salida
pub fn output_dir() -> PathBuf {
    project_dir().join("output")
}

/// Obtiene el directorio temporal
pub fn temp_dir() -> PathBuf {
    project_dir().join("temp")
}

/// Obtiene la ruta del archivo .env
pub fn env_path() -> PathBuf {
    project_dir().join(".env")
}

/// Obtiene el número máximo de workers
pub fn max_workers() -> usize {
    if let Ok(val) = env::var("MAX_WORKERS") {
        if let Ok(num) = val.parse::<usize>() {
            return num;
        }
    }
    
    let cpus = num_cpus();
    if cpus > 1 { cpus - 1 } else { 1 }
}

/// Obtiene el número de CPUs disponibles
pub fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

/// Verifica si el modo debug está habilitado
pub fn is_debug_mode() -> bool {
    env::var("DEBUG")
        .map(|v| v.to_lowercase() == "true" || v == "1")
        .unwrap_or(false)
}
