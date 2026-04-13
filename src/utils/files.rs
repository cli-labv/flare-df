//! FLARE-PDF - Utilidades de archivos
//! ===================================
//!
//! Operaciones de sistema de archivos.

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{input_dir, output_dir, temp_dir, env_path};

/// Asegura que existen los directorios necesarios
pub fn ensure_directories() -> Result<()> {
    for dir in &[input_dir(), output_dir(), temp_dir()] {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

/// Asegura que existe el archivo .env
pub fn ensure_env_file() -> Result<()> {
    let path = env_path();
    let example_path = crate::config::project_dir().join(".env.example");

    if !path.exists() {
        if example_path.exists() {
            fs::copy(&example_path, &path)?;
        } else {
            fs::write(&path, "SUDO_PASSWORD=\nDEBUG=false\n")?;
        }
    } else {
        let content = fs::read_to_string(&path)?;
        if !content.contains("SUDO_PASSWORD=") {
            let mut new_content = content;
            new_content.push_str("SUDO_PASSWORD=\n");
            fs::write(&path, new_content)?;
        }
    }
    Ok(())
}

/// Limpia y recrea el directorio temporal
pub fn reset_temp_directory() -> Result<()> {
    let temp = temp_dir();
    if temp.exists() {
        fs::remove_dir_all(&temp)?;
    }
    fs::create_dir_all(&temp)?;
    Ok(())
}

/// Limpia un directorio
#[allow(dead_code)]
pub fn cleanup_directory(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Obtiene el tamaño de un archivo de forma segura
pub fn safe_file_size(path: &Path) -> u64 {
    fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0)
}

/// Copia un archivo
#[allow(dead_code)]
pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(source, destination)?;
    Ok(())
}
