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

    fn custom_percent(&self) -> u8 {
        match self.mode {
            CompressionMode::Custom(percent) => percent.clamp(0, 99),
            _ => 0,
        }
    }

    fn lerp_u32(start: u32, end: u32, p: u8, p_start: u8, p_end: u8) -> u32 {
        if p_end <= p_start {
            return end;
        }
        let t = (p.saturating_sub(p_start)) as f64 / (p_end - p_start) as f64;
        let value = start as f64 + (end as f64 - start as f64) * t;
        value.round() as u32
    }

    fn custom_resolutions(&self) -> (u32, u32, u32) {
        let p = self.custom_percent();
        if p <= 30 {
            // Similar a Alta Calidad.
            return (300, 300, 1200);
        }
        if p <= 50 {
            // Transición Alta Calidad -> Balanceado.
            return (
                Self::lerp_u32(300, 150, p, 30, 50),
                Self::lerp_u32(300, 150, p, 30, 50),
                Self::lerp_u32(1200, 600, p, 30, 50),
            );
        }
        if p <= 60 {
            // Transición Balanceado -> Optimizado.
            return (
                Self::lerp_u32(150, 120, p, 50, 60),
                Self::lerp_u32(150, 120, p, 50, 60),
                Self::lerp_u32(600, 450, p, 50, 60),
            );
        }
        if p <= 70 {
            // Transición Optimizado -> Agresivo.
            return (
                Self::lerp_u32(120, 72, p, 60, 70),
                Self::lerp_u32(120, 72, p, 60, 70),
                Self::lerp_u32(450, 300, p, 60, 70),
            );
        }
        // Zona más fuerte que Agresivo.
        (
            Self::lerp_u32(72, 60, p, 70, 99),
            Self::lerp_u32(72, 60, p, 70, 99),
            Self::lerp_u32(300, 220, p, 70, 99),
        )
    }
    
    /// Obtiene el perfil de calidad según el modo de compresión
    fn get_pdfsettings(&self) -> &'static str {
        match self.mode {
            CompressionMode::Lossless => "/default", // No debería usarse, pero por seguridad
            CompressionMode::HighQuality => "/printer",
            CompressionMode::Balanced => "/ebook",
            CompressionMode::Optimized => "/ebook",
            CompressionMode::Aggressive => "/screen",
            CompressionMode::Custom(percent) => {
                if percent <= 40 {
                    "/printer"
                } else if percent <= 65 {
                    "/ebook"
                } else {
                    "/screen"
                }
            }
        }
    }
    
    /// Obtiene la resolución de imágenes en color según el modo
    fn get_color_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 300,
            CompressionMode::HighQuality => 300,
            CompressionMode::Balanced => 150,
            CompressionMode::Optimized => 120,
            CompressionMode::Aggressive => 72,
            CompressionMode::Custom(_) => self.custom_resolutions().0,
        }
    }
    
    /// Obtiene la resolución de imágenes en escala de grises según el modo
    fn get_gray_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 300,
            CompressionMode::HighQuality => 300,
            CompressionMode::Balanced => 150,
            CompressionMode::Optimized => 120,
            CompressionMode::Aggressive => 72,
            CompressionMode::Custom(_) => self.custom_resolutions().1,
        }
    }
    
    /// Obtiene la resolución de imágenes monocromáticas según el modo
    fn get_mono_image_resolution(&self) -> u32 {
        match self.mode {
            CompressionMode::Lossless => 1200,
            CompressionMode::HighQuality => 1200,
            CompressionMode::Balanced => 600,
            CompressionMode::Optimized => 450,
            CompressionMode::Aggressive => 300,
            CompressionMode::Custom(_) => self.custom_resolutions().2,
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
