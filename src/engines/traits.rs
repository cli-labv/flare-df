//! Trait para motores de compresión de PDFs
//! ========================================

use std::path::Path;
use anyhow::Result;

/// Trait que define la interfaz común para todos los motores de compresión
pub trait CompressionEngine {
    /// Nombre descriptivo del motor (ej: "QPDF", "Ghostscript")
    fn name(&self) -> &'static str;
    
    /// Versión del motor si está disponible
    fn version(&self) -> Result<String>;
    
    /// Verifica si el motor está instalado y disponible en el sistema
    fn is_available(&self) -> bool;
    
    /// Comprime un archivo PDF
    ///
    /// # Argumentos
    /// * `input_path` - Ruta del archivo PDF de entrada
    /// * `output_path` - Ruta donde guardar el PDF comprimido
    ///
    /// # Retorna
    /// * `Ok(())` si la compresión fue exitosa
    /// * `Err(...)` si hubo algún error durante el proceso
    fn compress(&self, input_path: &Path, output_path: &Path) -> Result<()>;
    
    /// Obtiene información sobre el tamaño del archivo
    ///
    /// # Argumentos
    /// * `path` - Ruta del archivo PDF
    ///
    /// # Retorna
    /// * Tamaño del archivo en bytes
    fn get_file_size(&self, path: &Path) -> Result<u64> {
        Ok(std::fs::metadata(path)?.len())
    }
    
    /// Calcula el porcentaje de reducción de tamaño
    ///
    /// # Argumentos
    /// * `original_size` - Tamaño original en bytes
    /// * `compressed_size` - Tamaño comprimido en bytes
    ///
    /// # Retorna
    /// * Porcentaje de reducción (ej: 25.5 significa 25.5% de reducción)
    fn calculate_reduction(&self, original_size: u64, compressed_size: u64) -> f64 {
        if original_size == 0 {
            return 0.0;
        }
        let reduction = original_size.saturating_sub(compressed_size) as f64;
        (reduction / original_size as f64) * 100.0
    }
}
