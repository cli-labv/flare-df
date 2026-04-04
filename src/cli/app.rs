//! FLARE-DF - Aplicación CLI principal
//! ====================================
//!
//! Define la aplicación principal.

use anyhow::Result;
use colored::Colorize;

use crate::cli::banner::{show_animated_banner, show_goodbye};
use crate::cli::menus::{MenuManager, DiagnosticDisplay};
use crate::config::{input_dir, max_workers};
use crate::core::{PdfScanner, CompressionProcessor};
use crate::models::{CompressionLevel, PdfTask, WorkMode};
use crate::utils::{
    ensure_directories, ensure_env_file, reset_temp_directory,
    PermissionManager,
};

/// Aplicación principal FLARE-DF
pub struct FlareApp {
    menu: MenuManager,
    display: DiagnosticDisplay,
    scanner: PdfScanner,
    permissions: PermissionManager,
}

impl FlareApp {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self {
            menu: MenuManager::new(),
            display: DiagnosticDisplay::new(),
            scanner: PdfScanner::new(),
            permissions: PermissionManager::new(),
        }
    }
    
    /// Inicializa el entorno de la aplicación
    fn initialize(&self) -> Result<()> {
        show_animated_banner();
        ensure_directories()?;
        ensure_env_file()?;
        reset_temp_directory()?;
        Ok(())
    }
    
    /// Verifica los permisos de escritura
    fn verify_permissions(&self) -> Result<()> {
        // Primero verificar silenciosamente
        let (success, _) = self.permissions.check_permissions_silent();
        
        if success {
            println!("{}", "✔ Permisos verificados".green());
            println!();
            return Ok(());
        }
        
        // Si falla, pedir contraseña interactivamente
        self.permissions.ensure_permissions_interactive()?;
        println!("{}", "✔ Permisos verificados".green());
        println!();
        Ok(())
    }
    
    /// Ejecuta la aplicación
    pub fn run(&self) -> Result<()> {
        // Capturar Ctrl+C
        ctrlc_handler();
        
        if let Err(e) = self.run_inner() {
            if e.to_string().contains("interrupted") {
                show_goodbye();
                return Ok(());
            }
            return Err(e);
        }
        
        Ok(())
    }
    
    fn run_inner(&self) -> Result<()> {
        self.initialize()?;
        self.verify_permissions()?;
        self.main_loop()
    }
    
    /// Bucle principal de la aplicación
    fn main_loop(&self) -> Result<()> {
        loop {
            // Seleccionar modo
            let mode = self.menu.select_mode()?;
            
            let base_path = match mode {
                WorkMode::Input => input_dir(),
                WorkMode::External(path) => path,
                WorkMode::Exit => {
                    show_goodbye();
                    return Ok(());
                }
            };
            
            // Escanear PDFs
            let tasks = match self.scan_with_retry(&base_path)? {
                Some(tasks) => tasks,
                None => continue, // Cambiar modo
            };
            
            if tasks.is_empty() {
                show_goodbye();
                return Ok(());
            }
            
            // Seleccionar nivel de compresión
            let level = self.menu.select_compression_level()?;
            
            // Actualizar tareas con el nivel seleccionado
            let tasks: Vec<PdfTask> = tasks
                .into_iter()
                .map(|t| t.with_level(level))
                .collect();
            
            // Mostrar diagnóstico
            self.display.show_scan_results(&tasks, &base_path, level);
            
            // Seleccionar estrategia de ejecución
            let parallel = self.menu.select_execution_strategy()?;
            
            // Confirmar compresión
            if !self.menu.confirm_compression(level)? {
                println!("{}", "Compresión cancelada por el usuario.".yellow());
                show_goodbye();
                return Ok(());
            }
            
            // Ejecutar compresión
            self.run_compression(tasks, parallel, level)?;
            break;
        }
        
        Ok(())
    }
    
    /// Escanea con opción de reintentar
    fn scan_with_retry(&self, base_path: &std::path::Path) -> Result<Option<Vec<PdfTask>>> {
        loop {
            let tasks = self.scanner.scan(base_path);
            
            if !tasks.is_empty() {
                return Ok(Some(tasks));
            }
            
            match self.menu.handle_empty_directory(base_path)? {
                "retry" => continue,
                "change_mode" => return Ok(None),
                _ => return Ok(Some(vec![])), // Exit
            }
        }
    }
    
    /// Ejecuta la compresión
    fn run_compression(
        &self,
        tasks: Vec<PdfTask>,
        parallel: bool,
        level: CompressionLevel,
    ) -> Result<()> {
        let workers = if parallel { max_workers() } else { 1 };
        let total_size: u64 = tasks.iter().map(|t| t.original_size).sum();
        
        self.display.show_compression_start(tasks.len(), total_size, workers, level);
        
        let style = self.menu.select_progress_style()?;
        let processor = CompressionProcessor::new(style);
        let summary = processor.process(tasks, parallel);
        
        // Limpiar temporales
        let _ = reset_temp_directory();
        
        self.display.show_compression_complete(&summary);
        
        if summary.has_failures() {
            std::process::exit(1);
        }
        
        Ok(())
    }
}

impl Default for FlareApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Configura el manejador de Ctrl+C
fn ctrlc_handler() {
    let _ = ctrlc::set_handler(move || {
        show_goodbye();
        std::process::exit(0);
    });
}
