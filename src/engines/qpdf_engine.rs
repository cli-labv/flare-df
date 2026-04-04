//! Motor de compresión usando QPDF
//! ================================
//!
//! QPDF es una herramienta potente para optimizaciones estructurales lossless de PDFs:
//! - Recompresión de streams con Flate óptimo
//! - Generación de object streams
//! - Eliminación de objetos duplicados
//! - Optimización de imágenes sin pérdida
//! - Limpieza de metadata innecesaria

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context, bail};
use crate::engines::traits::CompressionEngine;

/// Motor de compresión basado en QPDF
pub struct QpdfEngine;

impl QpdfEngine {
    /// Crea una nueva instancia del motor QPDF
    pub fn new() -> Self {
        Self
    }
    
    /// Ejecuta qpdf con parámetros óptimos para compresión lossless
    fn run_qpdf(&self, input: &Path, output: &Path) -> Result<()> {
        let output = Command::new("qpdf")
            // Recomprimir streams con compresión óptima
            .arg("--recompress-flate")
            // Generar object streams (reduce tamaño significativamente)
            .arg("--object-streams=generate")
            // Optimizar imágenes sin pérdida
            .arg("--optimize-images")
            // Forzar versión PDF 1.5 para mejor compatibilidad con object streams
            .arg("--force-version=1.5")
            // Linearizar el PDF (optimiza para streaming web)
            .arg("--linearize")
            // Normalizar contenido
            .arg("--normalize-content=y")
            // Comprimir streams
            .arg("--compress-streams=y")
            // Archivo de entrada
            .arg(input)
            // Archivo de salida
            .arg(output)
            .output()
            .context("Error ejecutando qpdf - asegúrate de que esté instalado")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("QPDF falló: {}", stderr);
        }
        
        Ok(())
    }
}

impl Default for QpdfEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressionEngine for QpdfEngine {
    fn name(&self) -> &'static str {
        "QPDF"
    }
    
    fn version(&self) -> Result<String> {
        let output = Command::new("qpdf")
            .arg("--version")
            .output()
            .context("No se pudo ejecutar qpdf --version")?;
        
        if !output.status.success() {
            bail!("qpdf --version falló");
        }
        
        let version_output = String::from_utf8_lossy(&output.stdout);
        // La primera línea suele contener la versión
        let version = version_output
            .lines()
            .next()
            .unwrap_or("desconocida")
            .to_string();
        
        Ok(version)
    }
    
    fn is_available(&self) -> bool {
        Command::new("qpdf")
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
        
        // Ejecutar qpdf
        self.run_qpdf(input_path, output_path)
            .context("Error durante la compresión con QPDF")?;
        
        Ok(())
    }
}
