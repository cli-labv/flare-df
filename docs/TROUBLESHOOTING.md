# 🐛 Solución de Problemas - FLARE-DF

Soluciones para problemas comunes.

## 🔴 Errores de Instalación

### Error: "rustc: command not found"
**Síntomas:** No puedes compilar el proyecto

**Solución:**
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verificar
rustc --version
```

---

### Error: "qpdf: command not found"
**Síntomas:** Mensaje "Motores faltantes: qpdf"

**Solución (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install qpdf

# Verificar
qpdf --version
```

**Solución (Fedora):**
```bash
sudo dnf install qpdf
```

**Solución (macOS):**
```bash
brew install qpdf
```

---

### Error: "ghostscript: command not found"
**Síntomas:** Falla compresión en Paso 2

**Solución (Ubuntu/Debian):**
```bash
sudo apt install ghostscript

# Verificar
gs --version
```

**Solución (Fedora):**
```bash
sudo dnf install ghostscript
```

**Solución (macOS):**
```bash
brew install ghostscript
```

---

## 🔴 Errores de Compilación

### Error: "linker error: could not compile"
**Síntomas:** Falla al compilar con `cargo build`

**Solución (Ubuntu/Debian):**
```bash
sudo apt install build-essential
cargo clean
cargo build --release
```

**Solución (Fedora):**
```bash
sudo dnf install gcc gcc-c++ make
cargo clean
cargo build --release
```

**Solución (macOS):**
```bash
xcode-select --install
cargo clean
cargo build --release
```

---

### Error: "multiple definitions of symbol"
**Síntomas:** Error de linkeo en compilación

**Solución:**
```bash
cargo clean
cargo build --release
```

---

## 🔴 Errores de Ejecución

### Error: "Permission denied" en carpeta
**Síntomas:** No puede escribir en carpeta output

**Solución 1: Configurar sudoers**
```bash
# Edita .env
nano .env

# Agrega
SUDO_PASSWORD=tu_contraseña

# Luego
./start.sh
```

**Solución 2: Ejecutar con sudo**
```bash
sudo ./start.sh
```

**Solución 3: Cambiar permisos**
```bash
chmod 777 output/
chmod 777 temp/
```

---

### Error: "Input path does not exist"
**Síntomas:** Carpeta no encontrada

**Solución:**
1. Verifica la ruta que ingresaste
2. Asegúrate que la carpeta existe:
   ```bash
   ls -la /ruta/a/carpeta
   ```
3. Si es modo external, ingresa la ruta absoluta completa

---

### Error: "No PDF files found"
**Síntomas:** Carpeta vacía o sin PDFs

**Solución:**
1. Verifica que hay PDFs en la carpeta:
   ```bash
   find ./input -name "*.pdf"
   ```
2. Si está vacía, coloca PDFs y reintenta
3. Usa modo external si los PDFs están en otra carpeta

---

### Error: "Failed to compress"
**Síntomas:** Falla durante compresión

**Solución:**
1. Verifica que el PDF no está dañado
2. Intenta con archivo diferente
3. Revisa permisos en output/
4. Intenta modo Lossless en lugar de agresivo

---

## 🔴 Problemas de Compresión

### PDF no se comprime
**Síntomas:** Reducción mínima (< 5%)

**Posibles causas:**
- PDF ya está optimizado
- Contiene muchas imágenes sin comprimir
- Nivel de compresión muy bajo

**Solución:**
1. Intenta nivel **Agresivo** en lugar de Balanceado
2. Intenta modo **Personalizado** con 80%
3. Si sigue igual, el PDF ya está optimizado

---

### Compresión es muy lenta
**Síntomas:** Tarda más de 10 minutos para 100 MB

**Posibles causas:**
- Modo secuencial
- Muchos PDFs
- Equipo con pocos recursos

**Solución:**
1. Usa **⚡ Paralelo** (mucho más rápido)
2. Reduce número de workers en `.env`
3. Cierra otras aplicaciones
4. Considera equipo más potente

---

### La salida se ve corrompida
**Síntomas:** Terminal con caracteres extraños

**Solución:**
```bash
# Reinicia terminal
exit

# O
clear
./start.sh
```

---

## 🟡 Advertencias (No son Errores)

### "Unused variable" en compilación
**Explicación:** Warnings normales de desarrollo

**Solución:** Ignóralos, no afectan funcionalidad

---

### "Deprecated function" en compilación
**Explicación:** Función antigua pero aún funcional

**Solución:** Ignóralos, será actualizado pronto

---

## 🟢 Casos Especiales

### Carpeta con muchos PDFs (1000+)
**Problema:** Puede tardar o usar mucha RAM

**Solución:**
```bash
# Modo secuencial puede ser más estable
# Selecciona: 🐢 Secuencial

# O ajusta workers
# En .env
MAX_WORKERS=2
```

---

### PDFs con contraseña
**Problema:** No se pueden procesar

**Solución actual:** No soportado

**Workaround:**
1. Desbloquea el PDF primero con otro tool
2. Luego comprímelo con FLARE-DF

---

### PDFs muy grandes (500+ MB)
**Problema:** Puede necesitar mucha RAM

**Solución:**
1. Aumenta RAM del sistema
2. Usa modo secuencial
3. Comprime por separado

---

### Caracteres especiales en nombres
**Problema:** Falla con acentos o caracteres Unicode

**Solución:**
1. Renombra archivos sin caracteres especiales
2. O usa solo ASCII: documento.pdf

---

## 🔧 Diagnóstico

### Ver versiones instaladas
```bash
# Rust
rustc --version

# QPDF
qpdf --version

# Ghostscript
gs --version

# FLARE-DF
./target/release/flare-df --version
```

### Ver configuración de FLARE-DF
```bash
cat .env
```

### Ver archivos temporales
```bash
ls -la temp/
```

### Limpiar archivos temporales
```bash
rm -rf temp/*
```

---

## 📞 Si Nada de Esto Funciona

1. **Recopila información:**
   ```bash
   rustc --version
   qpdf --version
   gs --version
   uname -a
   ```

2. **Revisa logs (si DEBUG=true):**
   ```bash
   cat ~/.flare-df/logs/
   ```

3. **Abre un issue en GitHub** con:
   - Descripción del problema
   - Pasos para reproducir
   - Salida de error completa
   - Información del sistema
   - Archivo `.env` (sin contraseña)

---

## 💡 Tips de Prevención

1. **Mantén herramientas actualizadas:**
   ```bash
   rustup update
   brew upgrade qpdf ghostscript
   ```

2. **Comprime primero un PDF pequeño:**
   - Verifica que todo funciona
   - Antes de hacer lotes grandes

3. **Haz backup de PDFs importantes:**
   - Aunque no se modifican, es buena práctica

4. **Usa diferentes niveles según el PDF:**
   - Prueba antes de comprimir muchos

---

**🔥 FLARE-DF** - Esperamos haber resuelto tu problema! 🎉
