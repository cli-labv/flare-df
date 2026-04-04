//! Detector de disponibilidad de motores de compresión
//! ====================================================

use crate::engines::{CompressionEngine, QpdfEngine, GhostscriptEngine, CompressionMode};
use anyhow::Result;
use colored::Colorize;

/// Información sobre la disponibilidad de un motor
#[derive(Debug, Clone)]
pub struct EngineAvailability {
    pub name: &'static str,
    pub available: bool,
    pub version: Option<String>,
}

impl EngineAvailability {
    /// Muestra información sobre el motor en la consola
    pub fn print_status(&self) {
        if self.available {
            let version_str = self.version
                .as_ref()
                .map(|v| format!("({})", v))
                .unwrap_or_default();
            println!(
                "  {} {} {}",
                "✓".green().bold(),
                self.name.green(),
                version_str.dimmed()
            );
        } else {
            println!(
                "  {} {} {}",
                "✗".red().bold(),
                self.name.red(),
                "(no instalado)".dimmed()
            );
        }
    }
    
    /// Muestra instrucciones de instalación si no está disponible
    pub fn print_installation_instructions(&self) {
        if !self.available {
            println!("\n{}", format!("Instrucciones para instalar {}:", self.name).yellow().bold());
            
            match self.name {
                "QPDF" => {
                    println!("  {}", "Ubuntu/Debian:".bold());
                    println!("    {}", "sudo apt install qpdf".cyan());
                    println!("  {}", "macOS:".bold());
                    println!("    {}", "brew install qpdf".cyan());
                    println!("  {}", "Fedora/RHEL:".bold());
                    println!("    {}", "sudo dnf install qpdf".cyan());
                }
                "Ghostscript" => {
                    println!("  {}", "Ubuntu/Debian:".bold());
                    println!("    {}", "sudo apt install ghostscript".cyan());
                    println!("  {}", "macOS:".bold());
                    println!("    {}", "brew install ghostscript".cyan());
                    println!("  {}", "Fedora/RHEL:".bold());
                    println!("    {}", "sudo dnf install ghostscript".cyan());
                }
                _ => {}
            }
        }
    }
}

/// Detector de motores de compresión disponibles
pub struct EngineDetector;

impl EngineDetector {
    /// Detecta qué motores están disponibles en el sistema
    pub fn detect_all() -> Vec<EngineAvailability> {
        vec![
            Self::detect_qpdf(),
            Self::detect_ghostscript(),
        ]
    }
    
    /// Detecta si QPDF está disponible
    pub fn detect_qpdf() -> EngineAvailability {
        let engine = QpdfEngine::new();
        let available = engine.is_available();
        let version = if available {
            engine.version().ok()
        } else {
            None
        };
        
        EngineAvailability {
            name: "QPDF",
            available,
            version,
        }
    }
    
    /// Detecta si Ghostscript está disponible
    pub fn detect_ghostscript() -> EngineAvailability {
        let engine = GhostscriptEngine::new(CompressionMode::Balanced);
        let available = engine.is_available();
        let version = if available {
            engine.version().ok()
        } else {
            None
        };
        
        EngineAvailability {
            name: "Ghostscript",
            available,
            version,
        }
    }
    
    /// Verifica si todos los motores necesarios para un modo están disponibles
    pub fn check_mode_requirements(mode: CompressionMode) -> Result<(), Vec<String>> {
        let required = mode.required_engines();
        let mut missing = Vec::new();
        
        for engine_name in required {
            let available = match engine_name {
                "qpdf" => Self::detect_qpdf().available,
                "ghostscript" => Self::detect_ghostscript().available,
                _ => false,
            };
            
            if !available {
                missing.push(engine_name.to_string());
            }
        }
        
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    
    /// Muestra un resumen completo de los motores disponibles
    pub fn print_summary() {
        println!("\n{}", "Motores de compresión disponibles:".bold());
        for engine in Self::detect_all() {
            engine.print_status();
        }
    }
    
    /// Muestra instrucciones de instalación para motores faltantes
    pub fn print_missing_installations() {
        let engines = Self::detect_all();
        let missing: Vec<_> = engines.iter().filter(|e| !e.available).collect();
        
        if !missing.is_empty() {
            println!("\n{}", "Motores no instalados:".yellow().bold());
            for engine in missing {
                engine.print_installation_instructions();
            }
        }
    }
}
