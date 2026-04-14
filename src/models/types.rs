//! FLARE-DF - Modelos y tipos de datos
//! =====================================
//!
//! Define las estructuras de datos utilizadas en toda la aplicación.

use std::path::PathBuf;

/// Nivel de compresión
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionLevel {
    /// 100% Lossless - Sin pérdida de calidad visual
    Lossless,
    /// Alta calidad - Mínima pérdida visual
    HighQuality,
    /// Balanceado - Buen equilibrio calidad/tamaño (recomendado)
    Balanced,
    /// Optimizado - Compresión alta manteniendo buena calidad visual
    Optimized,
    /// Agresivo - Máxima compresión
    Aggressive,
    /// Personalizado - El usuario especifica el porcentaje
    Custom(f64),
}

/// Organización de salida de PDFs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputLayout {
    Grouped,
    Flat,
}

impl Default for OutputLayout {
    fn default() -> Self {
        Self::Grouped
    }
}

impl CompressionLevel {
    /// Obtiene el nombre para mostrar
    pub fn display_name(&self) -> String {
        match self {
            Self::Lossless => "Lossless".to_string(),
            Self::HighQuality => "Alta Calidad".to_string(),
            Self::Balanced => "Balanceado".to_string(),
            Self::Optimized => "Optimizado".to_string(),
            Self::Aggressive => "Agresivo".to_string(),
            Self::Custom(percent) => format!("Personalizado ({}%)", percent),
        }
    }
    
    /// Obtiene la descripción
    pub fn description(&self) -> &'static str {
        match self {
            Self::Lossless => "100% sin pérdida - Solo optimizaciones estructurales",
            Self::HighQuality => "Compresión inteligente con mínima pérdida visual",
            Self::Balanced => "Equilibrio óptimo entre calidad y tamaño",
            Self::Optimized => "Compresión alta con buena calidad para PDFs con imágenes",
            Self::Aggressive => "Máxima compresión con calidad aceptable",
            Self::Custom(_) => "Nivel de compresión personalizado",
        }
    }
    
    /// Obtiene el factor de reducción estimado (porcentaje)
    pub fn estimated_reduction(&self) -> f64 {
        match self {
            Self::Lossless => 10.0,
            Self::HighQuality => 30.0,
            Self::Balanced => 50.0,
            Self::Optimized => 60.0,
            Self::Aggressive => 70.0,
            Self::Custom(percent) => *percent,
        }
    }
    
    /// Obtiene el emoji representativo
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Lossless => "💎",
            Self::HighQuality => "✨",
            Self::Balanced => "⚖️",
            Self::Optimized => "🛡️",
            Self::Aggressive => "🔥",
            Self::Custom(_) => "⚙️",
        }
    }
    
    /// Convierte a CompressionMode de engines
    pub fn to_engine_mode(&self) -> crate::engines::CompressionMode {
        use crate::engines::CompressionMode;
        match self {
            Self::Lossless => CompressionMode::Lossless,
            Self::HighQuality => CompressionMode::HighQuality,
            Self::Balanced => CompressionMode::Balanced,
            Self::Optimized => CompressionMode::Optimized,
            Self::Aggressive => CompressionMode::Aggressive,
            Self::Custom(percent) => {
                let clamped = percent.clamp(0.0, 99.0).round() as u8;
                CompressionMode::Custom(clamped)
            }
        }
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Representa un archivo PDF para comprimir
#[derive(Debug, Clone)]
pub struct PdfTask {
    pub source: PathBuf,
    pub display_name: String,
    pub original_size: u64,
    pub compression_level: CompressionLevel,
    pub target_path: Option<PathBuf>,
}

impl PdfTask {
    pub fn new(source: PathBuf, original_size: u64) -> Self {
        let display_name = source
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Self {
            source,
            display_name,
            original_size,
            compression_level: CompressionLevel::default(),
            target_path: None,
        }
    }
    
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.compression_level = level;
        self
    }

    pub fn with_target_path(mut self, target_path: PathBuf) -> Self {
        self.target_path = Some(target_path);
        self
    }
    
    /// Tamaño estimado después de compresión
    pub fn estimated_size(&self) -> u64 {
        let reduction = self.compression_level.estimated_reduction() / 100.0;
        ((1.0 - reduction) * self.original_size as f64) as u64
    }
    
    /// Porcentaje de reducción estimado
    pub fn estimated_reduction(&self) -> f64 {
        self.compression_level.estimated_reduction()
    }
    
    pub fn output_name(&self) -> String {
        format!("{}_compressed.pdf", self.display_name)
    }
}

/// Estilo de barra de progreso
#[derive(Debug, Clone)]
pub struct ProgressStyle {
    pub spinner: &'static str,
    pub bar_chars: &'static str,
    pub name: &'static str,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        Self {
            spinner: "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            bar_chars: "█▓▒░",
            name: "Flare 🔥",
        }
    }
}

/// Resultado de una compresión
#[derive(Debug)]
pub struct CompressionResult {
    pub name: String,
    pub success: bool,
    pub original_size: u64,
    pub final_size: u64,
    pub output_path: Option<PathBuf>,
    pub error: Option<String>,
}

impl CompressionResult {
    pub fn ok(name: String, original_size: u64, final_size: u64, output_path: PathBuf) -> Self {
        Self {
            name,
            success: true,
            original_size,
            final_size,
            output_path: Some(output_path),
            error: None,
        }
    }
    
    pub fn fail(name: String, original_size: u64, error: String) -> Self {
        Self {
            name,
            success: false,
            original_size,
            final_size: 0,
            output_path: None,
            error: Some(error),
        }
    }
    
    pub fn reduction_percent(&self) -> f64 {
        if self.original_size == 0 || !self.success {
            return 0.0;
        }
        ((self.original_size as f64 - self.final_size as f64) / self.original_size as f64) * 100.0
    }
}

/// Resumen de todas las compresiones
#[derive(Debug, Default)]
pub struct CompressionSummary {
    pub successful: Vec<String>,
    pub failed: Vec<(String, String)>,
    pub total_original: u64,
    pub total_compressed: u64,
}

impl CompressionSummary {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_success(&mut self, name: &str, original: u64, compressed: u64) {
        self.successful.push(name.to_string());
        self.total_original += original;
        self.total_compressed += compressed;
    }
    
    pub fn add_failure(&mut self, name: &str, error: &str) {
        self.failed.push((name.to_string(), error.to_string()));
    }
    
    pub fn success_count(&self) -> usize {
        self.successful.len()
    }
    
    pub fn failure_count(&self) -> usize {
        self.failed.len()
    }
    
    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }
    
    pub fn total_saved(&self) -> u64 {
        self.total_original.saturating_sub(self.total_compressed)
    }
    
    pub fn reduction_percent(&self) -> f64 {
        if self.total_original == 0 {
            return 0.0;
        }
        ((self.total_original as f64 - self.total_compressed as f64) / self.total_original as f64) * 100.0
    }
}

/// Modo de trabajo de la aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum WorkMode {
    Input,
    External(PathBuf),
    Exit,
}
