# 💻 API Reference - FLARE-DF

Referencia completa de la API interna de FLARE-DF.

## 📦 Módulos Públicos

### `engines` - Motores de Compresión

```rust
pub trait CompressionEngine {
    fn name(&self) -> &str;
    fn version(&self) -> String;
    fn is_available(&self) -> bool;
    fn compress(&self, input: &Path, output: &Path) -> Result<()>;
}

pub enum CompressionMode {
    Lossless,
    HighQuality,
    Balanced,
    Aggressive,
    Custom(f64),
}

pub struct QpdfEngine { }
pub struct GhostscriptEngine { }

pub fn compress_pdf(
    input: &Path,
    output: &Path,
    mode: CompressionMode,
) -> Result<CompressionStats>
```

### `core::scanner` - Escaneo de PDFs

```rust
pub struct PdfScanner {
    pub path: PathBuf,
}

impl PdfScanner {
    pub fn new(path: impl Into<PathBuf>) -> Self;
    pub fn scan(&self) -> Result<Vec<PathBuf>>;
    pub fn scan_recursive(&self) -> Result<Vec<PathBuf>>;
}
```

### `core::processor` - Procesamiento

```rust
pub struct CompressionProcessor {
    style: ProgressStyle,
}

impl CompressionProcessor {
    pub fn new(style: ProgressStyle) -> Self;
    pub fn process(
        &self,
        tasks: Vec<PdfTask>,
        parallel: bool,
    ) -> CompressionSummary;
}
```

### `models` - Tipos de Datos

```rust
pub struct PdfTask {
    pub input: PathBuf,
    pub output: PathBuf,
    pub original_size: u64,
}

pub struct CompressionResult {
    pub name: String,
    pub success: bool,
    pub original_size: u64,
    pub final_size: u64,
    pub error: Option<String>,
}

pub struct CompressionSummary {
    pub total_pdfs: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_original_size: u64,
    pub total_compressed_size: u64,
}
```

### `utils` - Utilidades

```rust
pub fn format_bytes(bytes: u64) -> String;
pub fn format_percentage(percent: f64) -> String;
pub fn format_duration(secs: u64) -> String;

pub struct PermissionManager {
    password: Option<String>,
}

impl PermissionManager {
    pub fn new() -> Self;
    pub fn ensure_permissions(&self, silent: bool) -> Result<()>;
}
```

---

## 🔧 Ejemplos de Uso

### Comprimir un PDF
```rust
use flare_df::engines::{compress_pdf, CompressionMode};
use std::path::Path;

fn main() -> Result<()> {
    let input = Path::new("input.pdf");
    let output = Path::new("output.pdf");
    
    let stats = compress_pdf(input, output, CompressionMode::Balanced)?;
    
    println!("Reducción: {:.1}%", stats.reduction_percentage);
    Ok(())
}
```

### Escanear PDFs
```rust
use flare_df::core::scanner::PdfScanner;

fn main() -> Result<()> {
    let scanner = PdfScanner::new("./input");
    let pdfs = scanner.scan_recursive()?;
    
    println!("PDFs encontrados: {}", pdfs.len());
    Ok(())
}
```

### Procesar en Paralelo
```rust
use flare_df::core::processor::CompressionProcessor;
use flare_df::models::types::{PdfTask, ProgressStyle};

fn main() -> Result<()> {
    let processor = CompressionProcessor::new(ProgressStyle::default());
    
    let tasks = vec![
        PdfTask { /* ... */ },
        PdfTask { /* ... */ },
    ];
    
    let summary = processor.process(tasks, true); // true = paralelo
    println!("Procesados: {}", summary.successful);
    Ok(())
}
```

---

## ⚙️ Estructuras Principales

### `CompressionStats`
```rust
pub struct CompressionStats {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub reduction_percentage: f64,
    pub mode: CompressionMode,
}

impl CompressionStats {
    pub fn format_size(bytes: u64) -> String;
    pub fn print(&self);
}
```

### `CompressionSummary`
```rust
pub struct CompressionSummary {
    pub total_pdfs: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_original_size: u64,
    pub total_compressed_size: u64,
    pub total_time: Duration,
}

impl CompressionSummary {
    pub fn total_reduction_percentage(&self) -> f64;
    pub fn throughput(&self) -> f64;
    pub fn print(&self);
}
```

---

## 📊 Enums

### `CompressionLevel`
```rust
pub enum CompressionLevel {
    Lossless,
    HighQuality,
    Balanced,
    Aggressive,
    Custom(f64),
}
```

### `CompressionMode`
```rust
pub enum CompressionMode {
    Lossless,        // QPDF solo
    HighQuality,     // QPDF + GS (300 dpi)
    Balanced,        // QPDF + GS (150 dpi)
    Aggressive,      // QPDF + GS (72 dpi)
    Custom(f64),     // Custom percentage
}
```

---

## 🔌 Traits

### `CompressionEngine`
```rust
pub trait CompressionEngine {
    /// Nombre del motor
    fn name(&self) -> &str;
    
    /// Versión disponible
    fn version(&self) -> String;
    
    /// Verificar disponibilidad
    fn is_available(&self) -> bool;
    
    /// Ejecutar compresión
    fn compress(
        &self,
        input: &Path,
        output: &Path,
    ) -> Result<()>;
}
```

---

## 🛠️ Funciones Útiles

### Validación
```rust
pub fn is_valid_pdf(path: &Path) -> bool;
pub fn is_valid_compression_level(percent: f64) -> bool;
pub fn is_valid_path(path: &Path) -> bool;
```

### Formateo
```rust
pub fn format_bytes(bytes: u64) -> String;
// "100 B", "1.5 KB", "50 MB", "2.3 GB"

pub fn format_percentage(percent: f64) -> String;
// "50.0%"

pub fn format_duration(secs: u64) -> String;
// "2s", "1m 30s", "2h 15m"
```

### Configuración
```rust
pub fn max_workers() -> usize;
pub fn get_temp_dir() -> PathBuf;
pub fn get_input_dir() -> PathBuf;
pub fn get_output_dir() -> PathBuf;
```

---

## ⚠️ Errores Posibles

### `anyhow::Error`

Los principales errores son:

```rust
// No encontrado
"PDF not found: /path/to/file.pdf"

// Permisos
"Permission denied: cannot write to output/"

// Motor faltante
"Motores faltantes: qpdf"

// Compresión fallida
"Error durante compresión con QPDF"

// Path inválido
"Invalid path: contains invalid characters"
```

---

## 🔄 Ciclo de Vida de Compresión

```
1. compress_pdf() - Punto de entrada
   ├─ Validar archivo existe
   ├─ Verificar motores disponibles
   └─ Obtener tamaño original

2. Crear temp directory

3. Según modo:
   ├─ Lossless:
   │   └─ qpdf_engine.compress()
   └─ Con pérdida:
       ├─ qpdf_engine.compress() → temp
       ├─ ghostscript_engine.compress() → output
       └─ Limpiar temp

4. Obtener tamaño comprimido

5. Calcular estadísticas

6. Retornar CompressionStats
```

---

## 📚 Documentación Inline

Todas las funciones públicas tienen documentación:

```rust
/// Comprime un archivo PDF
///
/// # Argumentos
/// * `input_path` - Ruta al PDF original
/// * `output_path` - Donde guardar el resultado
/// * `mode` - Nivel de compresión
///
/// # Retorna
/// * `Ok(CompressionStats)` con estadísticas
/// * `Err(...)` si algo falla
///
/// # Ejemplo
/// ```
/// use flare_df::engines::{compress_pdf, CompressionMode};
/// use std::path::Path;
///
/// let stats = compress_pdf(
///     Path::new("input.pdf"),
///     Path::new("output.pdf"),
///     CompressionMode::Balanced
/// )?;
/// ```
pub fn compress_pdf(
    input_path: &Path,
    output_path: &Path,
    mode: CompressionMode,
) -> Result<CompressionStats>
```

Accede a la documentación con:
```bash
cargo doc --open
```

---

**🔥 FLARE-DF** - API completa y bien documentada
