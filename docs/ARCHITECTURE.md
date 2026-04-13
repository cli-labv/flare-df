# 🏗️ Arquitectura - FLARE-DF

Descripción técnica de la arquitectura interna de FLARE-DF.

## 🎯 Visión General

FLARE-DF es una herramienta CLI modular construida en Rust que orquesta dos motores de compresión profesionales: QPDF y Ghostscript.

```
┌─────────────────────────────────────────────────┐
│  INTERFAZ CLI (cli/)                             │
│  ├─ Banner animado                              │
│  ├─ Menús interactivos                          │
│  └─ Visualización de resultados                 │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│  LÓGICA DE NEGOCIO (core/)                       │
│  ├─ Scanner (detecta PDFs)                      │
│  ├─ Processor (paralelo/secuencial)             │
│  └─ Compressor (orquestador)                    │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│  MOTORES DE COMPRESIÓN (engines/)                │
│  ├─ QpdfEngine (optimización estructural)       │
│  ├─ GhostscriptEngine (compresión inteligente)  │
│  ├─ Pipeline (orquestación)                     │
│  └─ Detector (verificación disponibilidad)      │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│  HERRAMIENTAS EXTERNAS                          │
│  ├─ qpdf (11.0+)                               │
│  └─ ghostscript (10.0+)                        │
└─────────────────────────────────────────────────┘
```

---

## 📁 Estructura de Módulos

```
src/
├── cli/                   # Interfaz de usuario
│   ├── app.rs            # Punto de entrada CLI
│   ├── banner.rs         # Banner ASCII animado
│   └── menus.rs          # Menús interactivos
│
├── core/                  # Lógica central
│   ├── compressor.rs     # Orquestador principal
│   ├── processor.rs      # Procesamiento paralelo/secuencial
│   └── scanner.rs        # Detección de PDFs
│
├── engines/              # Motores de compresión
│   ├── mod.rs           # Exports
│   ├── traits.rs        # Trait CompressionEngine
│   ├── qpdf_engine.rs   # Implementación QPDF
│   ├── ghostscript_engine.rs # Implementación Ghostscript
│   ├── pipeline.rs      # Orquestación de motores
│   ├── detector.rs      # Detección de herramientas
│   └── compression_mode.rs # Modos de compresión
│
├── config/               # Configuración
│   ├── settings.rs      # Constantes
│   └── styles.rs        # Estilos de UI
│
├── models/              # Tipos de datos
│   └── types.rs         # Estructuras
│
├── utils/               # Utilidades
│   ├── files.rs         # Operaciones de archivos
│   ├── formatting.rs    # Formateo de texto
│   └── permissions.rs   # Gestión de permisos
│
└── main.rs              # Punto de entrada
```

---

## 🔄 Flujo de Ejecución

### 1. Inicio
```
main.rs
  ↓
app.rs (procesa CLI args)
  ↓
banner.rs (muestra intro)
```

### 2. Interacción Usuaria
```
menus.rs
  ├─ Selecciona modo (input/external)
  ├─ Selecciona nivel compresión
  ├─ Selecciona paralelización
  └─ Confirmación
```

### 3. Escaneo
```
scanner.rs
  ├─ Detecta carpeta
  ├─ Encuentra PDFs recursivamente
  └─ Prepara lista de archivos
```

### 4. Diagnóstico
```
menus.rs (DiagnosticDisplay)
  ├─ Cuenta PDFs
  ├─ Calcula tamaño total
  ├─ Estima reducción
  └─ Muestra tabla detallada
```

### 5. Compresión
```
processor.rs
  ├─ Paralelo (Rayon ThreadPool)
  │   └─ Múltiples PDFs simultáneamente
  └─ Secuencial (uno a uno)
      └─ Con progreso detallado

       ↓

compressor.rs (compress_pdf_job)
  ├─ Verifica motores disponibles
  ├─ Llama pipeline.rs
  └─ Retorna estadísticas

       ↓

pipeline.rs (compress_pdf)
  ├─ Modo Lossless:
  │   └─ QPDF solo
  └─ Modos con pérdida:
      ├─ Paso 1: QPDF
      ├─ Paso 2: Ghostscript
      └─ Limpieza temp
```

### 6. Resultado
```
processor.rs / compressor.rs
  ├─ Calcula estadísticas finales
  ├─ Formatea salida
  └─ Muestra resumen
```

---

## 🎛️ Componentes Clave

### CLI Module (cli/)

**app.rs**
- Punto de entrada de la aplicación
- Procesa argumentos de línea de comandos
- Inicia el flujo principal

**banner.rs**
- Displays banner ASCII animado
- Estilos y colores
- Información de versión

**menus.rs**
- Menús interactivos con `dialoguer`
- Selección de modo
- Selección de compresión
- Diagnóstico y visualización

### Core Module (core/)

**scanner.rs**
- Escanea directorios recursivamente
- Filtra solo archivos PDF
- Valida archivos

**processor.rs**
- Gestiona procesamiento paralelo/secuencial
- Usa Rayon para paralelización
- Spinner de progreso
- Agregación de resultados

**compressor.rs**
- Orquestador principal
- Llamada a pipeline
- Calcula estadísticas
- Manejo de errores

### Engines Module (engines/)

**traits.rs**
```rust
pub trait CompressionEngine {
    fn name(&self) -> &str;
    fn compress(&self, input: &Path, output: &Path) -> Result<()>;
}
```

**qpdf_engine.rs**
- Implementa compresión lossless pura
- Usa comandos qpdf con parámetros óptimos
- Optimizaciones estructurales

**ghostscript_engine.rs**
- Implementa compresión con pérdida inteligente
- Perfiles: printer, ebook, screen
- Ajuste de DPI por nivel

**pipeline.rs**
- Orquesta secuencia de motores
- Lossless: QPDF solo
- Con pérdida: QPDF + Ghostscript
- Spinner aleatorio por paso
- Estadísticas finales

**detector.rs**
- Verifica disponibilidad de motores
- Proporciona instrucciones de instalación
- Detección por SO (Ubuntu, Fedora, macOS)

### Models Module (models/)

**types.rs**
- `CompressionLevel` - enum con 5 modos
- `PdfTask` - estructura de tarea
- `CompressionResult` - resultado de compresión
- `CompressionSummary` - resumen total

---

## 🔌 Integración de Motores

### Pipeline de Compresión

**Lossless Mode:**
```
Input PDF
   ↓
[QPDF Engine]
   ├─ --recompress-flate
   ├─ --object-streams=generate
   ├─ --optimize-images
   └─ --linearize
   ↓
Output PDF (5-15% menor)
```

**High Quality / Balanced / Aggressive:**
```
Input PDF
   ↓
[QPDF Engine]
   ├─ Optimizaciones estructurales
   ↓
[Temp File]
   ↓
[Ghostscript Engine]
   ├─ High Quality: /printer (300 dpi)
   ├─ Balanced: /ebook (150 dpi)
   ├─ Aggressive: /screen (72 dpi)
   └─ Optimización de imágenes
   ↓
Output PDF (20-80% menor)
```

---

## 🎨 Spinners Aleatorios

**Mecanismo:**
```rust
const SPINNERS: &[&str] = [...26 estilos...];

fn get_random_spinner() -> &'static str {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH);
    SPINNERS[seed % SPINNERS.len()]
}
```

**Resultado:**
- Diferente spinner cada ejecución
- Animación suave (80ms/frame)
- Mismo spinner para ambos pasos (consistencia)

---

## ⚡ Procesamiento Paralelo

**Rayon ThreadPool:**
```rust
let pool = ThreadPoolBuilder::new()
    .num_threads(max_workers())  // CPU count - 1
    .build()?;

pool.install(|| {
    tasks.par_iter().for_each(|task| {
        // Procesa en paralelo
    });
});
```

**Ventajas:**
- Utiliza todos los núcleos disponibles
- Deja 1 núcleo libre para el sistema
- Balanceo automático de carga

---

## 🛡️ Gestión de Errores

**Estrategia:**
- `anyhow::Result` para error propagation
- Contextos descriptivos en cada nivel
- Fallbacks cuando es posible
- Mensajes claros para el usuario

**Ejemplo:**
```rust
engine.compress(input, output)
    .context("Error durante la compresión con QPDF")?;
```

---

## 🔒 Seguridad

**Permisos:**
- Gestión automática de permisos sudo
- Contraseña desde `.env` (no en CLI)
- Fallback a prompt seguro
- Validación antes de operaciones

**Archivos:**
- Escaneo seguro de directorios
- Validación de rutas
- Auto-limpieza de temporales
- No modifica originales

---

## 📊 Estadísticas

**Recolección:**
- Tamaño original
- Tamaño comprimido
- Reducción porcentual
- Tiempo de procesamiento

**Presentación:**
- Tabla formateada
- Colores según resultado
- Resumen final

---

## 🔧 Configuración

**Variables de Entorno:**
- `SUDO_PASSWORD` - Para permisos
- `MAX_WORKERS` - Paralelización
- `DEBUG` - Modo debug

**Rutas Configurables:**
- `./input` - PDFs entrada
- `./output` - PDFs comprimidos
- `./temp` - Archivos temporales

---

## 🧪 Testing

**Suite de Tests:**
- CLI Alignment tests (renderizado)
- Tests de compresión
- Tests de permisos
- Tests de parallelization

---

## 📈 Flujo Completo Resumido

```
1. main.rs → Inicia
     ↓
2. banner.rs → Muestra intro
     ↓
3. menus.rs → Interacción usuario
     ↓
4. scanner.rs → Detecta PDFs
     ↓
5. menus.rs → Diagnóstico
     ↓
6. processor.rs → Inicia procesamiento
     ↓
7. compressor.rs → Orquesta compresión
     ↓
8. pipeline.rs → Ejecuta motores (QPDF + GS)
     ↓
9. processor.rs → Agrega resultados
     ↓
10. menus.rs → Muestra resumen final
```

---

**🔥 FLARE-DF Architecture** - Construido para eficiencia y extensibilidad
