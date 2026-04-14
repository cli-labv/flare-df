//! FLARE-DF - Menús interactivos
//! ==============================
//!
//! Maneja todos los menús y prompts del usuario.

use anyhow::Result;
use colored::Colorize;
use console::{Key, Term, measure_text_width};
use dialoguer::theme::ColorfulTheme;
use dialoguer::console::{style, Style};
use dialoguer::{Confirm, Input, Select};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::style_presets;
use crate::models::{CompressionLevel, CompressionSummary, OutputLayout, PdfTask, ProgressStyle, WorkMode};
use crate::utils::{format_bytes, format_percent, visual_width};

/// Gestor de menús
pub struct MenuManager {
    term: Term,
    theme: ColorfulTheme,
}

impl MenuManager {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            theme: ColorfulTheme {
                prompt_style: Style::new().white().bold(),
                active_item_style: Style::new().white().bold(),
                inactive_item_style: Style::new().white().bold(),
                checked_item_prefix: style("✓".to_string()).green().bold(),
                unchecked_item_prefix: style(" ".to_string()).white(),
                active_item_prefix: style("❯".to_string()).green().bold(),
                ..ColorfulTheme::default()
            },
        }
    }
    
    /// Muestra el menú de selección de modo
    pub fn select_mode(&self) -> Result<WorkMode> {
        let options = vec![
            "📂 Modo input (./input)".green().bold().to_string(),
            "📁 Modo external (ruta absoluta)".cyan().bold().to_string(),
            "🚪 Salir".red().bold().to_string(),
        ];

        loop {
            let selection = Select::with_theme(&self.theme)
                .with_prompt("🔥 Selecciona el modo de trabajo")
                .items(&options)
                .default(0)
                .interact_on(&self.term)?;

            match selection {
                0 => return Ok(WorkMode::Input),
                1 => match self.ask_external_path()? {
                    Some(path) => return Ok(WorkMode::External(path)),
                    None => continue,
                },
                _ => return Ok(WorkMode::Exit),
            }
        }
    }
    
    /// Muestra el menú de selección de nivel de compresión
    pub fn select_compression_level(&self) -> Result<CompressionLevel> {
        println!();
        let lines = vec![
            format!(
                "  {} {} - {}",
                pad_emoji("💎"),
                "Lossless".cyan(),
                "100% sin pérdida - Solo optimizaciones estructurales"
            ),
            format!(
                "  {} {} - {}",
                pad_emoji("✨"),
                "Alta Calidad".green(),
                "Compresión inteligente con mínima pérdida visual"
            ),
            format!(
                "  {}  {} - {}",
                pad_emoji("⚖️"),
                "Balanceado".yellow(),
                "Equilibrio óptimo calidad/tamaño (Recomendado)"
            ),
            format!(
                "  {}  {} - {}",
                pad_emoji("🛡️"),
                "Optimizado".purple(),
                "Compresión alta con buena calidad visual"
            ),
            format!(
                "  {} {} - {}",
                pad_emoji("🔥"),
                "Agresivo".red(),
                "Máxima compresión con calidad aceptable"
            ),
            format!(
                "  {}  {} - {}",
                pad_emoji("⚙️"),
                "Personalizado".white(),
                "Especifica tu propio porcentaje"
            ),
        ];
        let content_width = lines
            .iter()
            .map(|line| measure_text_width(line))
            .max()
            .unwrap_or(0)
            + 5;
        let border = "─".repeat(content_width);
        println!("{}", format!("╭{}╮", border).magenta());
        let title = "              🔧 NIVELES DE COMPRESIÓN";
        println!("{}", format!("│{}│", pad_to_width(title, content_width)).magenta());
        println!("{}", format!("├{}┤", border).magenta());
        for line in lines {
            println!("│{}│", pad_to_width(&line, content_width));
        }
        println!("{}", format!("╰{}╯", border).magenta());
        println!();
        
        let options = vec![
            "💎 Lossless (~10% reducción) - Sin pérdida visual".cyan().bold().to_string(),
            "✨ Alta Calidad (~30% reducción) - Mínima pérdida".green().bold().to_string(),
            "⚖️  Balanceado (~50% reducción) - Recomendado".yellow().bold().to_string(),
            "🛡️  Optimizado (~60% reducción) - Más compresión con buena calidad".purple().bold().to_string(),
            "🔥 Agresivo (~70% reducción) - Máxima compresión".red().bold().to_string(),
            "⚙️  Personalizado - Especifica tu porcentaje".white().bold().to_string(),
        ];
        
        let selection = self.select_list("🎯 Selecciona el nivel de compresión", &options, 2)?;

        let short_label = match selection {
            0 => "💎  Lossless",
            1 => "✨  Alta Calidad",
            2 => "⚖️  Balanceado",
            3 => "🛡️  Optimizado",
            4 => "🔥  Agresivo",
            5 => "⚙️  Personalizado",
            _ => "⚖️  Balanceado",
        };
        println!(
            "✔ 🎯 Selecciona el nivel de compresión · {}",
            short_label.white().bold()
        );
        
        match selection {
            0 => Ok(CompressionLevel::Lossless),
            1 => Ok(CompressionLevel::HighQuality),
            2 => Ok(CompressionLevel::Balanced),
            3 => Ok(CompressionLevel::Optimized),
            4 => Ok(CompressionLevel::Aggressive),
            5 => {
                // Modo personalizado - pedir porcentaje
                let percent_str: String = Input::with_theme(&self.theme)
                    .with_prompt("⚙️  Porcentaje de compresión deseado (70)")
                    .allow_empty(true)
                    .interact_text()?;
                
                let mut percent: f64 = if percent_str.trim().is_empty() {
                    70.0
                } else {
                    percent_str.trim().parse().unwrap_or(70.0)
                };
                
                // Limitar entre 0-99%
                if percent < 0.0 {
                    percent = 0.0;
                } else if percent > 99.0 {
                    percent = 99.0;
                }
                
                println!("✓ Compresión personalizada establecida en: {}%", percent);
                Ok(CompressionLevel::Custom(percent))
            }
            _ => Ok(CompressionLevel::Balanced),
        }
    }

    fn select_list(&self, prompt: &str, options: &[String], default: usize) -> Result<usize> {
        if !self.term.features().is_attended() || options.is_empty() {
            return Ok(default.min(options.len().saturating_sub(1)));
        }

        let mut current = default.min(options.len().saturating_sub(1));
        let mut rendered_lines = 0usize;

        loop {
            if rendered_lines > 0 {
                print!("\x1B[{}A", rendered_lines);
                print!("\x1B[J");
            }

            let prompt_line = format!("? {}", prompt);
            println!("{}", prompt_line);
            rendered_lines = count_wrapped_lines(&self.term, &prompt_line);

            for (idx, option) in options.iter().enumerate() {
                let line = if idx == current {
                    format!("  {} {}", "❯".green().bold(), option)
                } else {
                    format!("    {}", option)
                };
                println!("{}", line);
                rendered_lines += count_wrapped_lines(&self.term, &line);
            }

            std::io::stdout().flush()?;

            match self.term.read_key()? {
                Key::ArrowUp => {
                    if current == 0 {
                        current = options.len() - 1;
                    } else {
                        current -= 1;
                    }
                }
                Key::ArrowDown => {
                    current = (current + 1) % options.len();
                }
                Key::Enter => break,
                _ => {}
            }
        }

        Ok(current)
    }
    
    /// Muestra el menú de estrategia de ejecución
    pub fn select_execution_strategy(&self) -> Result<bool> {
        let options = vec![
            "⚡ Paralelo (rápido, recomendado)".green().bold().to_string(),
            "🔄 Secuencial (ver progreso detallado)".cyan().bold().to_string(),
        ];
        
        let selection = Select::with_theme(&self.theme)
            .with_prompt("⚙️  ¿Cómo deseas ejecutar la compresión?")
            .items(&options)
            .default(0)
            .interact_on(&self.term)?;
        
        Ok(selection == 0)
    }

    /// Muestra el menú de organización de salida
    pub fn select_output_layout(&self) -> Result<OutputLayout> {
        let options = vec![
            "📁 Ordenado por carpetas (mantener estructura)".cyan().bold().to_string(),
            "📄 Plano (todo directo en ./output)".green().bold().to_string(),
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("🧭 ¿Cómo deseas guardar los PDFs comprimidos?")
            .items(&options)
            .default(0)
            .interact_on(&self.term)?;

        Ok(match selection {
            1 => OutputLayout::Flat,
            _ => OutputLayout::Grouped,
        })
    }
    
    /// Muestra el menú de selección de estilo de progreso
    pub fn select_progress_style(&self) -> Result<ProgressStyle> {
        let presets = style_presets();
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as usize)
            .unwrap_or(0);
        let idx = seed % presets.len();
        Ok(presets[idx].1.clone())
    }
    
    /// Solicita una ruta externa
    pub fn ask_external_path(&self) -> Result<Option<PathBuf>> {
        let path_str: String = Input::with_theme(&self.theme)
            .with_prompt("📁 Ruta absoluta de la carpeta")
            .allow_empty(true)
            .interact_text()?;

        if path_str.trim().is_empty() {
            return Ok(None);
        }

        let path = PathBuf::from(path_str.trim());

        if !path.is_absolute() {
            println!("{}", "La ruta debe ser absoluta.".yellow());
            return Ok(None);
        }

        if !path.exists() {
            println!("{}", "La ruta indicada no existe.".yellow());
            return Ok(None);
        }

        if !path.is_dir() {
            println!("{}", "Debes indicar un directorio válido.".yellow());
            return Ok(None);
        }

        Ok(Some(path))
    }
    
    /// Maneja el caso de directorio vacío
    pub fn handle_empty_directory(&self, base_path: &std::path::Path) -> Result<&'static str> {
        println!();
        println!("{}", "╭─────────────────────────────────────────────╮".yellow());
        println!("{}", format!("│  🔥 No se encontraron PDFs en              │").yellow());
        println!("{}", format!("│  {:43}│", base_path.display()).yellow());
        println!("{}", "╰─────────────────────────────────────────────╯".yellow());
        println!();
        
        let options = vec![
            "🔄 Reintentar".green().bold().to_string(),
            "🔀 Cambiar modo".magenta().bold().to_string(),
            "🚪 Salir".red().bold().to_string(),
        ];
        
        let selection = Select::with_theme(&self.theme)
            .with_prompt("¿Qué deseas hacer?")
            .items(&options)
            .default(0)
            .interact_on(&self.term)?;
        
        match selection {
            0 => Ok("retry"),
            1 => Ok("change_mode"),
            _ => Ok("exit"),
        }
    }
    
    /// Solicita confirmación para iniciar
    pub fn confirm_compression(&self, level: CompressionLevel) -> Result<bool> {
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(format!(
                "🔥 Compresión {} - Se guardarán los PDFs en ./output. ¿Iniciar?",
                level.display_name()
            ))
            .default(true)
            .interact()?;
        
        Ok(confirmed)
    }
}

impl Default for MenuManager {
    fn default() -> Self {
        Self::new()
    }
}

fn count_wrapped_lines(term: &Term, line: &str) -> usize {
    let (_, cols) = term.size();
    let cols = if cols == 0 { 80 } else { cols as usize };
    let width = measure_text_width(line);
    let lines = (width + cols - 1) / cols;
    if lines == 0 { 1 } else { lines }
}

fn pad_to_width(text: &str, width: usize) -> String {
    let current = measure_text_width(text);
    if current >= width {
        text.to_string()
    } else {
        format!("{text}{}", " ".repeat(width - current))
    }
}

fn pad_emoji(emoji: &str) -> String {
    let width = measure_text_width(emoji);
    if width >= 2 {
        emoji.to_string()
    } else {
        format!("{emoji}{}", " ".repeat(2 - width))
    }
}


/// Visualizador de diagnósticos
pub struct DiagnosticDisplay;

impl DiagnosticDisplay {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self
    }
    
    /// Muestra los resultados del escaneo con nivel de compresión
    pub fn show_scan_results(&self, tasks: &[PdfTask], _base_path: &std::path::Path, level: CompressionLevel) {
        let total_size: u64 = tasks.iter().map(|t| t.original_size).sum();
        let reduction = level.estimated_reduction() / 100.0;
        let estimated_size = ((1.0 - reduction) * total_size as f64) as u64;
        let estimated_saved = total_size.saturating_sub(estimated_size);
        
        println!();
        println!("{}", "╭─────────────────────────────────────────────╮".green());
        println!("{}", "│              📊 RESUMEN                     │".green());
        println!("{}", "├─────────────────────────────────────────────┤".green());
        println!("│  📄 PDFs encontrados:  {:20} │", format!("{}", tasks.len()).cyan());
        println!("│  💾 Tamaño total:      {:20} │", format_bytes(total_size).cyan());
        println!("│  🎯 Nivel compresión:  {:20} │", 
            format!("{} {}", level.emoji(), level.display_name()).yellow()
        );
        println!("│  📉 Estimado después:  {:20} │", format_bytes(estimated_size).yellow());
        println!("│  ✨ Ahorro estimado:   {:20} │", 
            format!("{} (-{})", format_bytes(estimated_saved), format_percent(level.estimated_reduction())).green()
        );
        println!("{}", "╰─────────────────────────────────────────────╯".green());
        println!();
        
        // Tabla detallada
        println!("{}", "┌─────────────────────────────────────────────────────────────────┐".magenta());
        println!("{}", "│                    🔍 DIAGNÓSTICO DETALLADO                     │".magenta());
        println!("{}", "├─────────────────────────────┬──────────────┬──────────────┬─────┤".magenta());
        println!("{}", "│ Archivo                     │ Original     │ Estimado     │  %  │".magenta());
        println!("{}", "├─────────────────────────────┼──────────────┼──────────────┼─────┤".magenta());
        
        for task in tasks {
            let name = if task.display_name.len() > 27 {
                format!("{}...", &task.display_name[..24])
            } else {
                format!("{:27}", task.display_name)
            };
            
            let est_size = ((1.0 - reduction) * task.original_size as f64) as u64;
            
            println!(
                "│ {} │ {:>12} │ {:>12} │{:>4} │",
                name.cyan(),
                format_bytes(task.original_size),
                format_bytes(est_size).yellow(),
                format!("-{:.0}%", level.estimated_reduction()).green()
            );
        }
        
        println!("{}", "└─────────────────────────────┴──────────────┴──────────────┴─────┘".magenta());
        println!();
    }
    
    /// Muestra información al iniciar la compresión
    pub fn show_compression_start(&self, total_pdfs: usize, total_size: u64, workers: usize, level: CompressionLevel) {
        const BOX_WIDTH: usize = 67;
        
        println!();
        println!("{}", "╔═════════════════════════════════════════════════════════════════╗".red());
        
        // Título
        let title = "🔥 FLARE-DF iniciando compresión";
        let title_visual = visual_width(title);
        let title_padding = BOX_WIDTH.saturating_sub(title_visual + 4);
        println!("║  {}{}  ║", title.bright_red(), " ".repeat(title_padding));
        
        // Línea con PDFs, tamaño y workers
        let pdfs_str = format!("{}", total_pdfs);
        let size_str = format_bytes(total_size);
        let workers_str = format!("{}", workers);
        let content = format!("📄 PDFs: {} │ 💾 Tamaño: {} │ ⚡ Workers: {}", 
            pdfs_str.cyan(), size_str.cyan(), workers_str.yellow()
        );
        let content_visual = visual_width("📄 PDFs: ") + total_pdfs.to_string().len() + 
                             visual_width(" │ 💾 Tamaño: ") + size_str.len() + 
                             visual_width(" │ ⚡ Workers: ") + workers_str.len();
        let content_padding = BOX_WIDTH.saturating_sub(content_visual + 4);
        println!("║  {}{}  ║", content, " ".repeat(content_padding));
        
        // Línea con nivel de compresión
        let level_str = format!("{} {}", level.emoji(), level.display_name().green());
        let level_visual = visual_width("🎯 Nivel: ") + visual_width(level.emoji()) + 1 + level.display_name().len();
        let level_padding = BOX_WIDTH.saturating_sub(level_visual + 4);
        println!("║  🎯 Nivel: {}{}  ║", level_str, " ".repeat(level_padding));
        
        println!("{}", "╚═════════════════════════════════════════════════════════════════╝".red());
        println!();
    }
    
    /// Muestra el resultado final
    pub fn show_compression_complete(&self, summary: &CompressionSummary) {
        const BOX_WIDTH: usize = 67; // Ancho total del cuadro
        
        println!();
        println!("{}", "╔═════════════════════════════════════════════════════════════════╗".green());
        
        // Título
        let title = format!("🔥 FLARE-DF - Compresión completada");
        let title_visual = visual_width(&title);
        let title_padding = BOX_WIDTH.saturating_sub(title_visual + 4);
        println!("║  {}{}  ║", title.bright_green(), " ".repeat(title_padding));
        
        println!("║{}║", " ".repeat(BOX_WIDTH + 2));
        
        // PDFs comprimidos
        let pdfs_text = format!("📄 {} PDFs comprimidos", format!("{}", summary.success_count()).cyan());
        let pdfs_visual = visual_width("📄 ") + summary.success_count().to_string().len() + " PDFs comprimidos".len();
        let pdfs_padding = BOX_WIDTH.saturating_sub(pdfs_visual + 4);
        println!("║  {}{}  ║", pdfs_text, " ".repeat(pdfs_padding));
        
        // Tamaño
        let size_before = format_bytes(summary.total_original);
        let size_after = format_bytes(summary.total_compressed);
        let reduction = summary.reduction_percent();
        let reduction_str = format_percent(reduction.abs());
        let (sizes_text, sizes_visual) = if reduction >= 0.0 {
            let txt = format!(
                "💾 {} → {} (-{})",
                size_before.white(),
                size_after.green(),
                reduction_str.bright_green()
            );
            let visual = visual_width("💾 ") + size_before.len() + " → ".len() + size_after.len() + " (-".len() + reduction_str.len() + ")".len();
            (txt, visual)
        } else {
            let txt = format!(
                "💾 {} → {} (+{})",
                size_before.white(),
                size_after.red(),
                reduction_str.red()
            );
            let visual = visual_width("💾 ") + size_before.len() + " → ".len() + size_after.len() + " (+".len() + reduction_str.len() + ")".len();
            (txt, visual)
        };
        let sizes_padding = BOX_WIDTH.saturating_sub(sizes_visual + 4);
        println!("║  {}{}  ║", sizes_text, " ".repeat(sizes_padding));
        
        // Espacio ahorrado o incremento
        let (saved_text, saved_visual) = if summary.total_compressed <= summary.total_original {
            let saved = format_bytes(summary.total_saved());
            let text = format!("✨ Espacio ahorrado: {}", saved.bright_green());
            let visual = visual_width("✨ Espacio ahorrado: ") + saved.len();
            (text, visual)
        } else {
            let increase = summary.total_compressed - summary.total_original;
            let inc_str = format_bytes(increase);
            let text = format!("⚠️  Incremento tamaño: {}", inc_str.red());
            let visual = visual_width("⚠️  Incremento tamaño: ") + inc_str.len();
            (text, visual)
        };
        let saved_padding = BOX_WIDTH.saturating_sub(saved_visual + 4);
        println!("║  {}{}  ║", saved_text, " ".repeat(saved_padding));
        
        // Ubicación
        let location = "📂 Ubicación: ./output";
        let location_visual = visual_width(location);
        let location_padding = BOX_WIDTH.saturating_sub(location_visual + 4);
        println!("║  {}{}  ║", location, " ".repeat(location_padding));
        
        println!("{}", "╚═════════════════════════════════════════════════════════════════╝".green());
        
        if summary.has_failures() {
            println!();
            println!("{}", "⚠️  Algunos archivos fallaron:".yellow());
            for (name, error) in &summary.failed {
                println!("   • {}: {}", name.red(), error);
            }
        }
        
        println!();
    }
    
    /// Muestra un mensaje de error
    pub fn show_error(&self, message: &str) {
        println!("{} {}", "Error:".red(), message);
    }
}

impl Default for DiagnosticDisplay {
    fn default() -> Self {
        Self::new()
    }
}
