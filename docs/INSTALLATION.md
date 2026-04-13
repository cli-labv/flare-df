# 🚀 Guía de Instalación - FLARE-DF

Guía paso a paso para instalar FLARE-DF en tu sistema.

## 📋 Requisitos Previos

### Sistema Operativo
- Linux (Ubuntu, Debian, Fedora, etc.)
- macOS
- WSL2 en Windows

### Software Requerido
- **Rust 1.70+** (edición 2021)
- **QPDF 11.0+** - Motor de optimización estructural
- **Ghostscript 10.0+** - Motor de compresión
- **Git** - Control de versiones
- **curl** - Descargas (opcional)

---

## ⚙️ Instalación Paso a Paso

### 1️⃣ Instalar Rust

Si no tienes Rust instalado:

```bash
# Descargar e instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Activar Rust
source $HOME/.cargo/env

# Verificar instalación
rustc --version
cargo --version
```

### 2️⃣ Instalar Dependencias del Sistema

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    qpdf \
    ghostscript \
    git \
    build-essential \
    curl
```

#### Fedora/RHEL
```bash
sudo dnf install -y \
    qpdf \
    ghostscript \
    git \
    gcc \
    curl
```

#### macOS
```bash
# Requiere Homebrew (https://brew.sh)
brew install qpdf ghostscript git
```

### 3️⃣ Clonar el Repositorio

```bash
# Clonar FLARE-DF
git clone https://github.com/flare-df/flare-df.git
cd flare-df

# O si lo clonaste tu fork:
git clone https://github.com/TU_USUARIO/flare-df.git
cd flare-df
```

### 4️⃣ Configurar el Entorno

```bash
# Copiar configuración de ejemplo
cp .env.example .env

# Editar configuración (opcional)
nano .env

# Dar permisos de ejecución al script
chmod +x start.sh
```

### 5️⃣ Compilar el Proyecto

```bash
# Compilación en modo debug (rápida, para desarrollo)
cargo build

# O compilación en modo release (lenta, pero optimizada)
cargo build --release
```

### 6️⃣ Verificar Instalación

```bash
# Ejecutar el binario
./target/release/flare-df --help

# O usar el script de inicio
./start.sh
```

---

## ✅ Verificación de Instalación

### Verificar cada Componente

```bash
# ✅ Rust
rustc --version
# Debe mostrar: rustc X.X.X

# ✅ QPDF
qpdf --version
# Debe mostrar: qpdf version X.X.X

# ✅ Ghostscript
gs --version
# Debe mostrar: GPL Ghostscript X.X.X

# ✅ Git
git --version
# Debe mostrar: git version X.X.X
```

### Verificar FLARE-DF

```bash
cd /home/victor/code/flare-df
./target/release/flare-df --version
# Debe mostrar versión del proyecto
```

---

## 🎯 Primeros Pasos Después de Instalar

### Opción 1: Script Rápido
```bash
./start.sh
```

### Opción 2: Ejecución Directa
```bash
./target/release/flare-df
```

### Opción 3: Con Recompilación
```bash
cargo run --release
```

---

## 🐛 Solución de Problemas de Instalación

### Error: "rustc not found"
```bash
# Reinstalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Error: "qpdf not found"
```bash
# Ubuntu/Debian
sudo apt install qpdf

# Fedora
sudo dnf install qpdf

# macOS
brew install qpdf
```

### Error: "ghostscript not found"
```bash
# Ubuntu/Debian
sudo apt install ghostscript

# Fedora
sudo dnf install ghostscript

# macOS
brew install ghostscript
```

### Error: "permission denied" en start.sh
```bash
chmod +x start.sh
./start.sh
```

### Error al compilar: "linker error"
```bash
# Instalar dependencias de compilación
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc gcc-c++ make

# macOS
xcode-select --install
```

---

## 🔧 Configuración Después de Instalar

Ver [.env.example](../.env.example) para todas las opciones disponibles:

```env
# Contraseña sudo (si la necesitas)
SUDO_PASSWORD=tu_contraseña

# Workers para paralelización
MAX_WORKERS=4

# Modo debug
DEBUG=false
```

---

## 📦 Instalar desde Código Fuente (Desarrollo)

```bash
# Clonar tu fork
git clone https://github.com/TU_USUARIO/flare-df.git
cd flare-df

# Agregar upstream
git remote add upstream https://github.com/ORIGINAL/flare-df.git

# Instalar en modo desarrollo
cargo install --path .
```

---

## 🎉 ¡Instalación Completada!

Ahora puedes:
1. Leer [QUICKSTART.md](./QUICKSTART.md) para primeros pasos
2. Ver [EXAMPLES.md](./EXAMPLES.md) para casos de uso
3. Revisar [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) si tienes problemas

---

**🔥 FLARE-DF** - Instalado y listo para usar
