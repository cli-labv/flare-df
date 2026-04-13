//! FLARE-DF - Compresor de PDFs con motores externos
//! ==================================================
//!
//! Usa qpdf + ghostscript para compresión potente.

use anyhow::{Result, Context, bail};
use std::fs;

use crate::config::output_dir;
use crate::models::{CompressionResult, PdfTask};
use crate::engines::{compress_pdf, EngineDetector};

/// Compresor de PDFs usando motores externos
pub struct PdfCompressor;

impl PdfCompressor {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self
    }
    
    /// Verifica que los motores necesarios estén disponibles
    pub fn check_engines(&self) -> Result<()> {
        let qpdf = EngineDetector::detect_qpdf();
        
        if !qpdf.available {
            bail!("QPDF no está instalado. Instala con:\n  Ubuntu/Debian: sudo apt install qpdf\n  macOS: brew install qpdf");
        }
        
        // Ghostscript solo es necesario para modos no-lossless
        Ok(())
    }
    
    /// Comprime un PDF según el nivel especificado
    pub fn compress(&self, task: &PdfTask) -> CompressionResult {
        match self.do_compress(task) {
            Ok((final_size, output_path)) => {
                CompressionResult::ok(
                    task.display_name.clone(),
                    task.original_size,
                    final_size,
                    output_path,
                )
            }
            Err(e) => {
                CompressionResult::fail(
                    task.display_name.clone(),
                    task.original_size,
                    e.to_string(),
                )
            }
        }
    }
    
    fn do_compress(&self, task: &PdfTask) -> Result<(u64, std::path::PathBuf)> {
        // Convertir nivel a modo de motor
        let mode = task.compression_level.to_engine_mode();
        
        // Verificar motores necesarios
        if let Err(missing) = EngineDetector::check_mode_requirements(mode) {
            bail!(
                "Motores faltantes: {}. Por favor instálalos antes de continuar.",
                missing.join(", ")
            );
        }
        
        // Preparar ruta de salida
        let relative_output = task
            .target_path
            .clone()
            .unwrap_or_else(|| task.output_name().into());
        let output_path = output_dir().join(relative_output);
        
        // Asegurar que el directorio de salida existe
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Comprimir usando el pipeline de motores
        let stats = compress_pdf(&task.source, &output_path, mode)
            .context("Error durante la compresión")?;
        
        Ok((stats.compressed_size, stats.output_path))
    }
}

impl Default for PdfCompressor {
    fn default() -> Self {
        Self::new()
    }
}

/// Función standalone para procesamiento en paralelo
pub fn compress_pdf_job(task: PdfTask) -> CompressionResult {
    let compressor = PdfCompressor::new();
    compressor.compress(&task)
}
