# Sistema de Motores de Compresión - FLARE-DF

## 📋 Resumen

Nuevo sistema de motores de compresión con múltiples niveles: desde 100% lossless hasta compresión agresiva.

## �� Componentes Principales

### CompressionMode Enum
- **Lossless**: 100% sin pérdida (solo QPDF) → ~5-15%
- **HighQuality**: Mínima pérdida (QPDF + GS /printer 300dpi) → ~20-40%
- **Balanced**: Balance calidad/tamaño (QPDF + GS /ebook 150dpi) → ~40-60%
- **Aggressive**: Máxima compresión (QPDF + GS /screen 72dpi) → ~60-80%

### Motores Implementados

**QpdfEngine** (Lossless):
- Recompresión Flate óptima
- Object streams
- Optimización de imágenes
- Linearización

**GhostscriptEngine** (Compresión inteligente):
- Downsampling controlado
- Perfiles de calidad
- Compresión de fuentes
- Detección de duplicados

### Pipeline

**Lossless**: `Input → QPDF → Output`

**Otros modos**: `Input → QPDF → Temp → Ghostscript → Output`

## 🚀 Uso Básico

```rust
use flare_df::engines::{compress_pdf, CompressionMode, EngineDetector};

// Detectar motores
EngineDetector::print_summary();

// Comprimir
let stats = compress_pdf(input, output, CompressionMode::Balanced)?;
stats.print();
```

## 📦 Instalación

```bash
# Ubuntu/Debian
sudo apt install qpdf ghostscript

# macOS
brew install qpdf ghostscript
```

## 🔜 Próximos Pasos
1. Integrar al CLI
2. Procesamiento paralelo
3. Barras de progreso
4. Tests unitarios
