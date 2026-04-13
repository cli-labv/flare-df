//! FLARE-PDF - Procesador de compresiones
//! =======================================
//!
//! Maneja el procesamiento paralelo y secuencial.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle as IndicatifStyle};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::max_workers;
use crate::core::compress_pdf_job;
use crate::models::{CompressionSummary, PdfTask, ProgressStyle};
use crate::utils::format_bytes;

/// Procesador de compresiones
pub struct CompressionProcessor {
    style: ProgressStyle,
}

impl CompressionProcessor {
    /// Crea una nueva instancia
    pub fn new(style: ProgressStyle) -> Self {
        Self { style }
    }
    
    /// Procesa las tareas de compresión
    pub fn process(&self, tasks: Vec<PdfTask>, parallel: bool) -> CompressionSummary {
        if parallel && tasks.len() > 1 {
            self.process_parallel(tasks)
        } else {
            self.process_sequential(tasks)
        }
    }
    
    /// Procesamiento paralelo con rayon
    fn process_parallel(&self, tasks: Vec<PdfTask>) -> CompressionSummary {
        let summary = Arc::new(Mutex::new(CompressionSummary::new()));
        let multi = MultiProgress::new();
        
        let pb = multi.add(ProgressBar::new(tasks.len() as u64));
        pb.set_style(
            IndicatifStyle::default_bar()
                .template("{spinner:.red} [{bar:40.red/white}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .progress_chars(self.style.bar_chars)
                .tick_chars(self.style.spinner)
        );
        pb.set_message("🔥 Comprimiendo PDFs...");
        pb.enable_steady_tick(Duration::from_millis(100));
        
        // Configurar el pool de rayon
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(max_workers())
            .build()
            .unwrap();
        
        pool.install(|| {
            tasks.par_iter().for_each(|task| {
                let result = compress_pdf_job(task.clone());
                
                let mut sum = summary.lock().unwrap();
                if result.success {
                    sum.add_success(&result.name, result.original_size, result.final_size);
                    let _ = pb.println(format!(
                        "✔ {} (-{:.1}%)",
                        truncate_name(&result.name, 50),
                        result.reduction_percent()
                    ));
                } else {
                    sum.add_failure(&result.name, result.error.as_deref().unwrap_or("Error"));
                    let _ = pb.println(format!("✖ {} → Error", truncate_name(&result.name, 50)));
                }
                pb.inc(1);
            });
        });
        
        pb.finish_with_message("✅ Compresión completada");
        
        Arc::try_unwrap(summary)
            .unwrap_or_else(|_| panic!("Failed to unwrap summary"))
            .into_inner()
            .unwrap()
    }
    
    /// Procesamiento secuencial con progreso detallado
    fn process_sequential(&self, tasks: Vec<PdfTask>) -> CompressionSummary {
        let mut summary = CompressionSummary::new();
        let total = tasks.len();
        
        for (idx, task) in tasks.into_iter().enumerate() {
            println!("\n🔥 [{}/{}] {}", idx + 1, total, task.display_name);
            println!("─────────────────────────────────────────────────");
            println!("   📄 Tamaño original: {}", format_bytes(task.original_size));
            
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                IndicatifStyle::default_spinner()
                    .template("{spinner:.red} {msg}")
                    .unwrap()
                    .tick_chars(self.style.spinner)
            );
            pb.set_message("Comprimiendo...");
            pb.enable_steady_tick(Duration::from_millis(80));
            
            let result = compress_pdf_job(task);
            
            pb.finish_and_clear();
            
            if result.success {
                summary.add_success(&result.name, result.original_size, result.final_size);
                println!(
                    "   ✔ Comprimido: {} → {} (-{:.1}%)",
                    format_bytes(result.original_size),
                    format_bytes(result.final_size),
                    result.reduction_percent()
                );
            } else {
                let error = result.error.unwrap_or_else(|| "Error desconocido".to_string());
                summary.add_failure(&result.name, &error);
                println!("   ✖ Error: {}", error);
            }
        }
        
        summary
    }
}

fn truncate_name(name: &str, max: usize) -> String {
    if name.chars().count() <= max {
        return name.to_string();
    }
    let mut out: String = name.chars().take(max.saturating_sub(3)).collect();
    out.push_str("...");
    out
}
