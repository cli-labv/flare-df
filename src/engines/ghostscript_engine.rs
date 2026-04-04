//! Motor de compresión usando Ghostscript
//! =======================================
//!
//! Ghostscript proporciona compresión inteligente de PDFs con diferentes perfiles:
//! - /screen: Máxima compresión (72 dpi) para visualización en pantalla
//! - /ebook: Compresión moderada (150 dpi) para libros electrónicos
//! - /printer: Alta calidad (300 dpi) para impresión
//! - /prepress: Máxima calidad (300+ dpi) para imprenta profesional

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context, bail};
use crate::engines::traits::CompressionEngine;
use crate::engines::compression_mode::CompressionMode;

/// Motor de compresión basado en Ghostscript
pub struct GhostscriptEngine {
    mode: CompressionMode,
}

impl GhostscriptEngine {
    /// Crea una nueva instancia del motor Ghostscript
    pub fn new(mode: CompressionMode) -> Self {
        Self { mode }
    }
    
    /// Obtiene el perfil de calidad según el modo de compresión
    fn get_pdfsettings(&self) -> &'static str {
        match self.mode {
            CompressionMode::Lossless => "/default", // No debería usarse, pero por seguridad
            CompressionMode::HighQuality => "/printer",
            CompressionMode::Balanced => "/ebook",
            CompressionMode::Aggressive => "/screen",
        }
    }
    
    /// Obtiene la resolución de imágenes en color según el modo
    fn get_color_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 300,
            CompressionMode::HighQuality => 300,
            CompressionMode::Balanced => 150,
            CompressionMode::Aggressive => 72,
        }
    }
    
    /// Obtiene la resolución de imágenes en escala de grises según el modo
    fn get_gray_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 300,
            CompressionMode::HighQuality => 300,
            CompressionMode::Balanced => 150,
            CompressionMode::Aggressive => 72,
        }
    }
    
    /// Obtiene la resolución de imágenes monocromáticas según el modo
    fn get_mono_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 1200,
            CompressionMode::HighQuality => 1200,
            CompressionMode::Balanced => 600,
            CompressionMode::Aggressive => 300,
        }
    }
    
    /// Ejecuta Ghostscript con parámetros óptimos según el modo
    fn run_ghostscript(&self, input: &Path, output: &Path) -> Result<()> {
        let pdfsettings = self.get_pdfsettings();
        let color_res = self.get_color_image_resolution();
        let gray_res = self.get_gray_image_resolution();
        let mono_res = self.get_mono_image_resolution();
        
        let output = Command::new("gs")
            // Modo batch sin interfaz
            .arg("-sDEVICE=pdfwrite")
            .arg("-dNOPAUSE")
            .arg("-dQUIET")
            .arg("-dBATCH")
            // Perfil de calidad
            .arg(format!("-dPDFSETTINGS={}", pdfsettings))
            // Compatibilidad PDF 1.5
            .arg("-dCompatibilityLevel=1.5")
            // Resoluciones para imágenes en color
            .arg(format!("-dColorImageResolution={}", color_res))
            .arg("-dDownsampleColorImages=true")
            .arg("-dColorImageDownsampleType=/Bicubic")
            // Resoluciones para imágenes en escala de grises
            .arg(format!("-dGrayImageResolution={}", gray_res))
            .arg("-dDownsampleGrayImages=true")
            .arg("-dGrayImageDownsampleType=/Bicubic")
            // Resoluciones para imágenes monocromáticas
            .arg(format!("-dMonoImageResolution={}", mono_res))
            .arg("-dDownsampleMonoImages=true")
            // Compresión de imágenes
            .arg("-dAutoFilterColorImages=true")
            .arg("-dAutoFilterGrayImages=true")
            // Optimizaciones adicionales
            .arg("-dDetectDuplicateImages=true")
            .arg("-dCompressFonts=true")
            .arg("-dSubsetFonts=true")
            // Archivo de salida
            .arg(format!("-sOutputFile={}", output.display()))
            // Archivo de entrada
            .arg(input)
            .output()
            .context("Error ejecutando Ghostscript - asegúrate de que esté instalado")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Ghostscript falló: {}", stderr);
        }
        
        Ok(())
    }
}

impl CompressionEngine for GhostscriptEngine {
    fn name(&self) -> &'static str {
        "Ghostscript"
    }
    
    fn version(&self) -> Result<String> {
        let output = Command::new("gs")
            .arg("--version")
            .output()
            .context("No se pudo ejecutar gs --version")?;
        
        if !output.status.success() {
            bail!("gs --version falló");
        }
        
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        
        Ok(version)
    }
    
    fn is_available(&self) -> bool {
        Command::new("gs")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    fn compress(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Verificar que el archivo de entrada existe
        if !input_path.exists() {
            bail!("El archivo de entrada no existe: {}", input_path.display());
        }
        
        // Crear directorio de salida si no existe
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Error creando directorio de salida")?;
        }
        
        // Ejecutar Ghostscript
        self.run_ghostscript(input_path, output_path)
            .context("Error durante la compresión con Ghostscript")?;
        
        Ok(())
    }
}
