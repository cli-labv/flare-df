# 🔥 FLARE-DF

> **Compresor de PDFs ultrarrápido y 100% lossless**

FLARE-DF es una herramienta CLI en Rust para comprimir archivos PDF de forma lossless (sin pérdida de calidad). Usa **QPDF** como motor de compresión, ofreciendo múltiples niveles de optimización.

## ✨ Características

- 🚀 **Procesamiento paralelo** con Rayon
- 📄 **Compresión 100% lossless** - sin pérdida de calidad
- 🎯 **3 niveles de compresión**: Mínima, Normal, Extrema
- 🎨 **Barras de progreso animadas** con indicatif
- 📂 **Dos modos de trabajo**: carpeta local (`./input`) o ruta externa
- 📊 **Diagnóstico previo** con estimación de reducción
- 🔐 **Gestión automática de permisos** (sudo desde `.env`)
- 🦀 **Powered by QPDF** - motor profesional de manipulación PDF

## 🎯 Niveles de Compresión

| Nivel | Reducción | Descripción |
|-------|-----------|-------------|
| 📦 **Mínima** | ~10% | Limpieza básica, object streams, linearización |
| ⚡ **Normal** | ~25% | Optimización equilibrada, Flate óptima, sin duplicados |
| 🔥 **Extrema** | ~40% | Todas las optimizaciones agresivas disponibles |

## 📦 Estructura del Proyecto

```
flare-df/
├── src/
│   ├── cli/           # Interfaz de línea de comandos
│   │   ├── app.rs     # Aplicación principal
│   │   ├── banner.rs  # Banner animado ASCII
│   │   └── menus.rs   # Menús interactivos
│   ├── core/          # Lógica de negocio
│   │   ├── compressor.rs  # Compresión con QPDF
│   │   ├── processor.rs   # Procesamiento paralelo/secuencial
│   │   └── scanner.rs     # Escáner de archivos
│   ├── config/        # Configuración
│   │   ├── settings.rs    # Constantes y rutas
│   │   └── styles.rs      # Estilos de progreso
│   ├── models/        # Modelos de datos
│   │   └── types.rs       # Estructuras de datos
│   ├── utils/         # Utilidades
│   │   ├── files.rs       # Operaciones de archivos
│   │   ├── formatting.rs  # Formateo de texto
│   │   └── permissions.rs # Gestión de permisos
│   └── main.rs        # Punto de entrada
├── input/             # Carpeta de PDFs de entrada
├── output/            # PDFs comprimidos
├── temp/              # Archivos temporales
├── .env               # Variables de entorno
├── .env.example       # Plantilla de configuración
├── Cargo.toml         # Dependencias
└── README.md
```

## 🚀 Instalación y Uso

### Requisitos

- Rust 1.70+ (edición 2021)
- QPDF instalado en el sistema (`apt install qpdf` o `brew install qpdf`)
- Sistema Unix/Linux (o WSL en Windows)

### Compilación

```bash
cd flare-df

# Compilar en modo release
cargo build --release

# Ejecutar
./target/release/flare-df
```

### Uso con script de inicio

```bash
chmod +x start.sh
./start.sh
```

## 🔧 Configuración

Copia `.env.example` a `.env` y configura:

```env
# Contraseña sudo (opcional)
SUDO_PASSWORD=tu_contraseña

# Workers para procesamiento paralelo
MAX_WORKERS=4

# Modo debug
DEBUG=false
```

## 📝 Flujo de Trabajo

1. **Selección de modo**: `input` (carpeta local) o `external` (ruta absoluta)
2. **Escaneo**: Detecta todos los PDFs recursivamente
3. **Nivel de compresión**: Mínima / Normal / Extrema
4. **Diagnóstico**: Muestra tamaños y estimaciones
5. **Ejecución**: Paralelo o secuencial
6. **Confirmación**: Pide confirmación antes de procesar
7. **Resultado**: PDFs en `./output` con sufijo `_compressed.pdf`

## 📄 Licencia

MIT License

---

**🔥 FLARE-DF** - Hecho con 🦀 Rust + QPDF
