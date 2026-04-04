//! Ejemplo de uso del sistema de compresión de PDFs
//! =================================================

use anyhow::Result;

fn main() -> Result<()> {
    println!("🔥 FLARE-DF - Demostración de motores de compresión\n");
    
    println!("=== Ejemplo de uso ===");
    println!();
    println!("1. Detectar motores:");
    println!("   use flare_df::engines::EngineDetector;");
    println!("   EngineDetector::print_summary();");
    println!();
    println!("2. Comprimir un PDF:");
    println!("   use flare_df::engines::{{compress_pdf, CompressionMode}};");
    println!("   let stats = compress_pdf(input, output, CompressionMode::Balanced)?;");
    println!("   stats.print();");
    
    Ok(())
}
