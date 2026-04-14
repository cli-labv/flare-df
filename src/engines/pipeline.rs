//! Pipeline de compresión de PDFs
//! ===============================
//!
//! Este módulo coordina la compresión de PDFs usando diferentes motores
//! según el modo de compresión seleccionado.

use std::path::{Path, PathBuf};
use anyhow::{Result, Context, bail};
use colored::Colorize;
use crate::engines::{
    CompressionEngine,
    CompressionMode,
    QpdfEngine,
    GhostscriptEngine,
    EngineDetector,
};

/// Estadísticas de compresión de un archivo
#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub reduction_percentage: f64,
    pub mode: CompressionMode,
}

impl CompressionStats {
    /// Formatea el tamaño en formato legible (KB, MB, GB)
    pub fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", bytes)
        }
    }
    
    /// Muestra las estadísticas de compresión
    pub fn print(&self) {
        println!("\n{}", "Resultado de compresión:".bold());
        println!("  {}: {}", "Modo".bold(), self.mode);
        println!("  {}: {}", "Tamaño original".bold(), Self::format_size(self.original_size));
        println!("  {}: {}", "Tamaño comprimido".bold(), Self::format_size(self.compressed_size));
        
        let reduction_color = if self.reduction_percentage > 50.0 {
            "green"
        } else if self.reduction_percentage > 20.0 {
            "yellow"
        } else {
            "red"
        };
        
        let reduction_str = format!("{:.1}%", self.reduction_percentage);
        println!(
            "  {}: {}",
            "Reducción".bold(),
            match reduction_color {
                "green" => reduction_str.green(),
                "yellow" => reduction_str.yellow(),
                _ => reduction_str.red(),
            }
        );
        
        let saved = self.original_size.saturating_sub(self.compressed_size);
        println!("  {}: {}", "Espacio ahorrado".bold(), Self::format_size(saved).green());
    }
}

/// Comprime un archivo PDF usando el pipeline apropiado según el modo
///
/// # Argumentos
/// * `input_path` - Ruta del archivo PDF de entrada
/// * `output_path` - Ruta donde guardar el PDF comprimido
/// * `mode` - Modo de compresión a utilizar
///
/// # Pipeline según modo:
/// - **Lossless**: Solo QPDF (optimizaciones estructurales)
/// - **HighQuality**: QPDF → Ghostscript (/printer, 300 dpi)
/// - **Balanced**: QPDF → Ghostscript (/ebook, 150 dpi)
/// - **Optimized**: QPDF → Ghostscript (/ebook, 120 dpi)
/// - **Aggressive**: QPDF → Ghostscript (/screen, 72 dpi)
///
/// # Retorna
/// * `Ok(CompressionStats)` con las estadísticas de compresión
/// * `Err(...)` si hubo algún error durante el proceso
pub fn compress_pdf(
    input_path: &Path,
    output_path: &Path,
    mode: CompressionMode,
) -> Result<CompressionStats> {
    // Verificar que el archivo existe
    if !input_path.exists() {
        bail!("El archivo no existe: {}", input_path.display());
    }
    
    // Verificar que los motores requeridos están disponibles
    if let Err(missing) = EngineDetector::check_mode_requirements(mode) {
        bail!(
            "Motores faltantes: {}. Por favor instálalos antes de continuar.",
            missing.join(", ")
        );
    }
    
    // Obtener tamaño original
    let original_size = std::fs::metadata(input_path)
        .context("Error leyendo metadata del archivo original")?
        .len();
    
    // Crear directorio temporal para archivos intermedios
    let temp_dir = output_path.parent()
        .unwrap_or_else(|| Path::new("."))
        .join("temp");
    std::fs::create_dir_all(&temp_dir)
        .context("Error creando directorio temporal")?;
    
    let final_output = match mode {
        CompressionMode::Lossless => {
            // Solo QPDF - compresión lossless pura
            let engine = QpdfEngine::new();
            engine.compress(input_path, output_path)
                .context("Error durante la compresión con QPDF")?;
            output_path.to_path_buf()
        }
        
        CompressionMode::HighQuality
        | CompressionMode::Balanced
        | CompressionMode::Optimized
        | CompressionMode::Aggressive
        | CompressionMode::Custom(_) => {
            // Pipeline: QPDF primero para optimizaciones estructurales
            let qpdf_output = temp_dir.join("qpdf_output.pdf");
            let qpdf_engine = QpdfEngine::new();
            qpdf_engine.compress(input_path, &qpdf_output)
                .context("Error durante la optimización con QPDF")?;
            
            // Luego Ghostscript para compresión inteligente
            let gs_engine = GhostscriptEngine::new(mode);
            gs_engine.compress(&qpdf_output, output_path)
                .context("Error durante la compresión con Ghostscript")?;
            
            // Limpiar archivo temporal
            let _ = std::fs::remove_file(qpdf_output);
            
            output_path.to_path_buf()
        }
    };
    
    // Obtener tamaño comprimido
    let compressed_size = std::fs::metadata(&final_output)
        .context("Error leyendo metadata del archivo comprimido")?
        .len();
    
    // Calcular reducción
    let reduction_percentage = if original_size > 0 {
        ((original_size.saturating_sub(compressed_size)) as f64 / original_size as f64) * 100.0
    } else {
        0.0
    };
    
    // Limpiar directorio temporal si está vacío
    let _ = std::fs::remove_dir(&temp_dir);
    
    Ok(CompressionStats {
        input_path: input_path.to_path_buf(),
        output_path: final_output,
        original_size,
        compressed_size,
        reduction_percentage,
        mode,
    })
}

/// Comprime múltiples archivos PDF usando el mismo modo
///
/// # Argumentos
/// * `files` - Lista de tuplas (input_path, output_path)
/// * `mode` - Modo de compresión a utilizar
///
/// # Retorna
/// * Vector con las estadísticas de cada archivo comprimido
pub fn compress_multiple_pdfs(
    files: Vec<(PathBuf, PathBuf)>,
    mode: CompressionMode,
) -> Vec<Result<CompressionStats>> {
    files
        .into_iter()
        .map(|(input, output)| compress_pdf(&input, &output, mode))
        .collect()
}
