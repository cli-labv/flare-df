# 📋 Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [1.0.0] - 2026-04-04

### 🎉 Lanzamiento Inicial

Primera versión estable de FLARE-DF con soporte completo para compresión de PDFs usando motores profesionales.

### ✨ Añadido

#### Compresión Multi-Motor
- Sistema de compresión dual: QPDF + Ghostscript
- 5 niveles de compresión:
  - 💎 **Lossless** (~5-15% reducción) - 100% sin pérdida
  - ✨ **Alta Calidad** (~20-40% reducción) - 300 dpi
  - ⚖️ **Balanceado** (~40-60% reducción) - 150 dpi (Recomendado)
  - 🔥 **Agresivo** (~60-80% reducción) - 72 dpi
  - ⚙️ **Personalizado** (0-99%) - Especifica tu porcentaje

#### Motores de Compresión
- `QpdfEngine`: Optimizaciones estructurales lossless
  - Object streams
  - Linearización
  - Recompresión Flate
  - Normalización de contenido
- `GhostscriptEngine`: Compresión inteligente
  - Perfiles de calidad: /printer, /ebook, /screen
  - Optimización de imágenes
  - Reducción de DPI configurable
- `CompressionPipeline`: Orquestación de motores
  - Pipeline de 2 pasos para máxima compresión
  - Estadísticas detalladas
  - Gestión automática de archivos temporales

#### Interfaz CLI Mejorada
- Banner ASCII animado con estilo
- Menús interactivos con `dialoguer`
- 26 estilos de spinners aleatorios
  - Dots, lines, arcs, squares
  - Arrows, triangles, growth
  - Emojis: moon 🌑, clock 🕐
- Diagnóstico detallado pre-compresión
  - Cantidad de PDFs
  - Tamaño total
  - Estimación de reducción
  - Ahorro proyectado

#### Procesamiento
- Modo paralelo con Rayon (multi-core)
- Modo secuencial con progreso detallado
- Configuración automática de workers (CPU count - 1)
- Procesamiento por lotes eficiente

#### Gestión de Archivos
- Dos modos de trabajo:
  - 📂 Input mode (carpeta local `./input`)
  - 📁 External mode (ruta absoluta)
- Escaneo recursivo de directorios
- Auto-limpieza de archivos temporales
- Preservación de estructura de carpetas

#### Seguridad y Permisos
- Gestión automática de permisos sudo
- Lectura de contraseña desde `.env`
- Fallback a prompt CLI si no está configurada
- Validación de permisos antes de operaciones

#### Documentación
- README completo con ejemplos
- SPINNERS.md explicando sistema de animación
- COMPRESSION_ENGINES.md con detalles técnicos
- CONTRIBUTING.md guía para contribuidores
- .env.example con instrucciones detalladas

#### Tests
- Suite de tests CLI Alignment
- 10+ métodos para renderizado perfecto de terminales
- Soporte para emojis y caracteres anchos
- 4 escenarios de demostración

### 🔧 Cambios Técnicos

#### Arquitectura
- Sistema modular con trait-based engines
- Separación clara de responsabilidades:
  - `cli/` - Interfaz de usuario
  - `core/` - Lógica de negocio
  - `engines/` - Motores de compresión
  - `models/` - Tipos de datos
  - `utils/` - Utilidades compartidas
  - `config/` - Configuración

#### Dependencias
- `clap` - CLI argument parsing
- `colored` - Salida con colores
- `indicatif` - Progress bars y spinners
- `dialoguer` - Menús interactivos
- `rayon` - Procesamiento paralelo
- `walkdir` - Traversal de directorios
- `dotenv` - Variables de entorno
- `anyhow` - Error handling
- `bytesize` - Formateo de tamaños

### 🛠️ Infraestructura

#### Build System
- Compilación optimizada en release mode
  - `opt-level = 3`
  - `lto = true`
  - `codegen-units = 1`
  - `strip = true`
- Script `start.sh` para inicio rápido

#### Git
- `.gitignore` completo y organizado
  - Archivos de build
  - IDE configurations
  - Secrets (.env)
  - Temporales
  - PDFs (input/output)
  - Logs
- `.gitkeep` en carpetas necesarias

### 📝 Documentación

- README.md actualizado con:
  - Guía de instalación completa
  - Tabla de niveles de compresión
  - Ejemplos de uso
  - Estructura del proyecto
  - Troubleshooting
- LICENSE (MIT)
- CONTRIBUTING.md
- CHANGELOG.md (este archivo)

### 🎨 UX/UI

- Salida colorida y bien formateada
- Boxes y tablas alineados perfectamente
- Emojis para mejor legibilidad
- Spinners animados durante procesamiento
- Resumen final con estadísticas

### 🔍 Detección de Herramientas

- Auto-detección de QPDF y Ghostscript
- Instrucciones de instalación por sistema operativo
- Validación antes de ejecutar compresión
- Mensajes de error claros y accionables

## [0.1.0] - Desarrollo Inicial

### Añadido
- Estructura básica del proyecto
- Compresión simple con QPDF
- CLI básico

---

## Leyenda de Tipos de Cambios

- `✨ Añadido` - Nueva funcionalidad
- `🔧 Cambiado` - Cambios en funcionalidad existente
- `🗑️ Deprecado` - Funcionalidad que se eliminará pronto
- `🗑️ Eliminado` - Funcionalidad eliminada
- `🐛 Corregido` - Bug fixes
- `🔒 Seguridad` - Fixes de seguridad

---

**🔥 FLARE-DF** - Compresión profesional de PDFs con Rust
