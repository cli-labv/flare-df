# 🔥 FLARE-DF

> **Compresor de PDFs ultrarrápido con motores profesionales de compresión**

FLARE-DF es una herramienta CLI escrita en Rust que comprime archivos PDF usando motores profesionales (**QPDF** + **Ghostscript**), ofreciendo desde compresión 100% lossless hasta reducciones agresivas de hasta 80%.

## 🚀 Inicio Rápido

```bash
# Clonar
git clone <repo>
cd flare-df

# Configurar
cp .env.example .env
chmod +x start.sh

# Ejecutar
./start.sh
```

👉 **Primeros pasos:** Lee [docs/QUICKSTART.md](docs/QUICKSTART.md)

---

## ✨ Características

- 🎯 **5 Niveles de Compresión**: Lossless, Alta Calidad, Balanceado, Agresivo, Personalizado
- 🔥 **Doble Motor**: QPDF (optimización estructural) + Ghostscript (compresión inteligente)
- 🎨 **Spinners Aleatorios** - 26 estilos diferentes en cada ejecución
- ⚡ **Procesamiento Paralelo** - Comprime múltiples PDFs simultáneamente
- 📊 **Diagnóstico Detallado** - Estimación de reducción antes de empezar
- 🔐 **Gestión de Permisos** - Automática vía `.env`
- 📁 **Dos Modos**: input local o carpeta externa
- 🏆 **Resultados Reales**: 10-80% de reducción según el nivel

---

## 🎯 Niveles de Compresión

| Nivel | Icono | Reducción | Descripción |
|-------|-------|-----------|-------------|
| **Lossless** | 💎 | ~5-15% | 100% sin pérdida - Optimizaciones estructurales |
| **Alta Calidad** | ✨ | ~20-40% | Compresión inteligente con mínima pérdida visual |
| **Balanceado** | ⚖️ | ~40-60% | Equilibrio óptimo calidad/tamaño **(Recomendado)** |
| **Agresivo** | 🔥 | ~60-80% | Máxima compresión con calidad aceptable |
| **Personalizado** | ⚙️ | 0-99% | Especifica tu propio porcentaje (default: 70%) |

---

## 📚 Documentación Completa

| Tema | Documento | Para |
|------|-----------|------|
| 🚀 Empezar rápido | [QUICKSTART.md](docs/QUICKSTART.md) | Todos |
| 📦 Instalación | [INSTALLATION.md](docs/INSTALLATION.md) | Nuevos usuarios |
| 🏗️ Arquitectura | [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Desarrolladores |
| 💻 API Reference | [API.md](docs/API.md) | Desarrolladores |
| 🎨 Spinners | [SPINNERS.md](docs/SPINNERS.md) | Curiosos |
| ⚙️ Motores | [COMPRESSION_ENGINES.md](docs/COMPRESSION_ENGINES.md) | Técnico |
| 📚 Ejemplos | [EXAMPLES.md](docs/EXAMPLES.md) | Usuarios |
| 🧪 Testing | [TESTING.md](docs/TESTING.md) | Desarrolladores |
| ❓ FAQ | [FAQ.md](docs/FAQ.md) | Todos |
| 🐛 Troubleshooting | [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) | Problemas |
| 🤝 Contribuir | [CONTRIBUTING.md](docs/CONTRIBUTING.md) | Desarrolladores |
| 📋 Changelog | [CHANGELOG.md](docs/CHANGELOG.md) | Todos |

👉 **Índice completo:** [docs/README.md](docs/README.md)

---

## ⚙️ Requisitos

### Sistema Operativo
- Linux (Ubuntu, Debian, Fedora, etc.)
- macOS
- WSL2 en Windows

### Software Requerido
- **Rust 1.70+** (edición 2021)
- **QPDF 11.0+** - Motor de optimización
- **Ghostscript 10.0+** - Motor de compresión

### Instalación de Dependencias

#### Ubuntu/Debian
```bash
sudo apt install -y qpdf ghostscript
```

#### Fedora
```bash
sudo dnf install -y qpdf ghostscript
```

#### macOS
```bash
brew install qpdf ghostscript
```

👉 **Detalles completos:** [docs/INSTALLATION.md](docs/INSTALLATION.md)

---

## 🚀 Instalación

### 1. Clonar Repositorio
```bash
git clone https://github.com/flare-df/flare-df.git
cd flare-df
```

### 2. Configurar Entorno
```bash
cp .env.example .env
nano .env  # Opcional: configura SUDO_PASSWORD
```

### 3. Compilar
```bash
cargo build --release
```

### 4. Ejecutar
```bash
./start.sh
```

---

## 📝 Uso Básico

### 1️⃣ Ejecuta la App
```bash
./start.sh
```

### 2️⃣ Selecciona Modo
```
📂 Modo input (./input)
📁 Modo external (ruta absoluta)
```

### 3️⃣ Elige Nivel de Compresión
```
💎 Lossless (~10%)
✨ Alta Calidad (~30%)
⚖️  Balanceado (~50%)  ← Recomendado
🔥 Agresivo (~70%)
⚙️  Personalizado (custom)
```

### 4️⃣ Selecciona Ejecución
```
⚡ Paralelo (rápido)   ← Recomendado
🐢 Secuencial (detallado)
```

### 5️⃣ Confirma
```
¿Iniciar? yes
```

---

## 📂 Estructura del Proyecto

```
flare-df/
├── 📖 Documentación
│   ├── README.md
│   └── docs/
│       ├── README.md          (índice)
│       ├── QUICKSTART.md
│       ├── INSTALLATION.md
│       ├── ARCHITECTURE.md
│       ├── API.md
│       ├── SPINNERS.md
│       ├── COMPRESSION_ENGINES.md
│       ├── EXAMPLES.md
│       ├── TESTING.md
│       ├── FAQ.md
│       ├── TROUBLESHOOTING.md
│       ├── CONTRIBUTING.md
│       └── CHANGELOG.md
│
├── 💻 Código Fuente
│   ├── src/
│   │   ├── cli/          (interfaz)
│   │   ├── core/         (lógica)
│   │   ├── engines/      (motores)
│   │   ├── models/       (tipos)
│   │   ├── utils/        (utilidades)
│   │   ├── config/       (config)
│   │   └── main.rs
│   ├── examples/
│   └── tests/
│
├── 📂 Directorios de Trabajo
│   ├── input/    (PDFs entrada)
│   ├── output/   (PDFs comprimidos)
│   └── temp/     (temporales)
│
├── ⚙️ Configuración
│   ├── .env
│   ├── .env.example
│   ├── .gitignore
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── start.sh
│
├── 📄 Licencia
│   └── LICENSE (MIT)
│
└── 🔧 Control de Versiones
    └── .git
```

---

## 🔧 Configuración

Edita `.env` para personalizar:

```env
# Contraseña sudo (para permisos)
SUDO_PASSWORD=

# Workers para paralelización
MAX_WORKERS=

# Modo debug
DEBUG=false
```

Ver [.env.example](.env.example) para todas las opciones.

---

## 🎨 Ejemplo de Uso

```bash
$ ./start.sh

🔥 Selecciona el modo de trabajo:
> 📂 Modo input (./input)

🎯 Selecciona el nivel de compresión:
> ⚖️  Balanceado (~50% reducción)

╭─────────────────────────────┐
│ 📊 RESUMEN                  │
├─────────────────────────────┤
│ 📄 PDFs: 3                  │
│ 💾 Tamaño: 300 MB           │
│ 📉 Estimado: 150 MB         │
│ ✨ Ahorro: 150 MB (-50%)    │
╰─────────────────────────────╯

⚙️  Ejecución: ⚡ Paralelo
🔥 ¿Iniciar? yes

[Comprimiendo...]

╔═════════════════════════════╗
║ 🔥 FLARE-DF Completado     ║
║ 📄 3 PDFs comprimidos      ║
║ 💾 300 MB → 150 MB (-50%)  ║
║ 📂 ./output/               ║
╚═════════════════════════════╝
```

---

## ⚡ Características Avanzadas

### Spinners Aleatorios
En cada ejecución ves un spinner diferente:
```
⠋⠙⠹⠸  (dots)
▉▊▋▌  (grow)
◐◓◑◒  (arc)
🌑🌒🌓🌔  (moon)
```
👉 Ver [docs/SPINNERS.md](docs/SPINNERS.md)

### Pipeline de Compresión
**Lossless:** Input → QPDF → Output
**Con pérdida:** Input → QPDF → Temp → Ghostscript → Output

👉 Ver [docs/COMPRESSION_ENGINES.md](docs/COMPRESSION_ENGINES.md)

### Procesamiento Paralelo
Usa todos los núcleos disponibles para máxima velocidad.
```bash
# Configurable en .env
MAX_WORKERS=4
```

---

## 🧪 Testing

```bash
# Todos los tests
cargo test

# Tests específicos
cargo test --test cli_alignment

# Con output detallado
cargo test -- --nocapture
```

👉 Ver [docs/TESTING.md](docs/TESTING.md)

---

## 🐛 Solución de Problemas

### Error: "qpdf not found"
```bash
sudo apt install qpdf
```

### Error: "ghostscript not found"
```bash
sudo apt install ghostscript
```

### Error: "Permission denied"
Configura `SUDO_PASSWORD` en `.env`

👉 **Guía completa:** [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)

---

## ❓ Preguntas Frecuentes

**¿Pierdo calidad?**
No en modo Lossless. Mínima en Alta Calidad. Imperceptible en Balanceado.

**¿Funciona en Windows?**
Sí, con WSL2 (Windows Subsystem for Linux).

**¿Cuánto tarda?**
1-5 minutos para 100 MB en modo paralelo.

**¿Puedo especificar mi porcentaje?**
Sí, usa modo Personalizado (0-99%).

👉 **Más FAQs:** [docs/FAQ.md](docs/FAQ.md)

---

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! 

1. Fork el proyecto
2. Crea una rama (`git checkout -b feature/amazing`)
3. Commit cambios (`git commit -m 'Add amazing feature'`)
4. Push (`git push origin feature/amazing`)
5. Abre un Pull Request

👉 **Guía completa:** [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE) para detalles

---

## 🙏 Agradecimientos

- **QPDF** - Excelente herramienta de manipulación PDF
- **Ghostscript** - Motor profesional de compresión
- **Rust Community** - Por las increíbles librerías

---

## 📞 Soporte

- 📖 **Docs:** [docs/README.md](docs/README.md)
- 🆘 **Problemas:** [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)
- ❓ **FAQ:** [docs/FAQ.md](docs/FAQ.md)
- 🐛 **Issues:** GitHub Issues
- 💬 **Preguntas:** Abre una Discussion

---

**🔥 FLARE-DF** - Compresión profesional de PDFs con Rust

⭐ Si te gusta este proyecto, dale una estrella en GitHub!
