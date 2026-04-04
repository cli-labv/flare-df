//! FLARE-PDF - Escáner de archivos PDF
//! =====================================
//!
//! Detecta y analiza archivos PDF.

use std::path::Path;
use walkdir::WalkDir;

use crate::config::PDF_EXTENSIONS;
use crate::models::PdfTask;
use crate::utils::safe_file_size;

/// Escáner de archivos PDF
pub struct PdfScanner;

impl PdfScanner {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self
    }
    
    /// Verifica si un archivo es un PDF
    pub fn is_pdf_file(path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| {
                let ext = format!(".{}", e);
                PDF_EXTENSIONS.iter().any(|pe| pe.eq_ignore_ascii_case(&ext))
            })
            .unwrap_or(false)
    }
    
    /// Escanea un directorio buscando PDFs
    pub fn scan(&self, base_path: &Path) -> Vec<PdfTask> {
        let mut tasks = Vec::new();
        
        for entry in WalkDir::new(base_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() && Self::is_pdf_file(path) {
                let size = safe_file_size(path);
                if size > 0 {
                    tasks.push(PdfTask::new(path.to_path_buf(), size));
                }
            }
        }
        
        tasks.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        tasks
    }
    
    /// Verifica si hay PDFs en la ruta especificada
    pub fn has_pdfs(&self, base_path: &Path) -> bool {
        !self.scan(base_path).is_empty()
    }
}

impl Default for PdfScanner {
    fn default() -> Self {
        Self::new()
    }
}
