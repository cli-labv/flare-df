//! Modos de compresión disponibles
//! ================================

use std::fmt;

/// Niveles de compresión con diferentes balances entre calidad y tamaño
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    /// Compresión 100% lossless - máxima calidad, sin pérdida visual
    /// Solo usa optimizaciones estructurales (qpdf)
    Lossless,
    
    /// Alta calidad - muy buena calidad visual con reducción moderada
    /// Usa qpdf + Ghostscript con perfil /printer o /ebook + 200-300 dpi
    HighQuality,
    
    /// Balanceado - buen balance entre calidad y tamaño
    /// Usa qpdf + Ghostscript con perfil /ebook + resolución controlada
    Balanced,

    /// Optimizado - más compresión que balanceado, manteniendo buena calidad
    /// Usa qpdf + Ghostscript con downsampling moderado
    Optimized,
    
    /// Agresivo - máxima compresión con calidad aceptable
    /// Usa qpdf + Ghostscript con perfil /screen + downsampling fuerte
    Aggressive,

    /// Personalizado - compresión variable según porcentaje del usuario
    /// Usa qpdf + Ghostscript con parámetros calculados dinámicamente
    Custom(u8),
}

impl CompressionMode {
    /// Retorna todos los modos disponibles
    pub fn all() -> Vec<Self> {
        vec![
            Self::Lossless,
            Self::HighQuality,
            Self::Balanced,
            Self::Optimized,
            Self::Aggressive,
        ]
    }
    
    /// Retorna una descripción del modo
    pub fn description(&self) -> &'static str {
        match self {
            Self::Lossless => "100% lossless - Solo optimizaciones estructurales (máxima calidad)",
            Self::HighQuality => "Alta calidad - Compresión inteligente con mínima pérdida visual",
            Self::Balanced => "Balanceado - Buen equilibrio calidad/tamaño (recomendado)",
            Self::Optimized => "Optimizado - Compresión alta con buena calidad visual",
            Self::Aggressive => "Agresivo - Máxima compresión con calidad aceptable",
            Self::Custom(_) => "Personalizado - Compresión variable según porcentaje",
        }
    }
    
    /// Retorna la reducción estimada de tamaño
    pub fn estimated_reduction(&self) -> &'static str {
        match self {
            Self::Lossless => "~5-15%",
            Self::HighQuality => "~20-40%",
            Self::Balanced => "~40-60%",
            Self::Optimized => "~50-70%",
            Self::Aggressive => "~60-80%",
            Self::Custom(_) => "Variable (según porcentaje)",
        }
    }
    
    /// Retorna si el modo usa solo herramientas lossless
    pub fn is_lossless(&self) -> bool {
        matches!(self, Self::Lossless)
    }
    
    /// Retorna los motores requeridos para este modo
    pub fn required_engines(&self) -> Vec<&'static str> {
        match self {
            Self::Lossless => vec!["qpdf"],
            _ => vec!["qpdf", "ghostscript"],
        }
    }
}

impl fmt::Display for CompressionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Lossless => "Lossless",
            Self::HighQuality => "High Quality",
            Self::Balanced => "Balanced",
            Self::Optimized => "Optimized",
            Self::Aggressive => "Aggressive",
            Self::Custom(percent) => return write!(f, "Custom ({}%)", percent),
        };
        write!(f, "{}", name)
    }
}

impl Default for CompressionMode {
    fn default() -> Self {
        Self::Balanced
    }
}
